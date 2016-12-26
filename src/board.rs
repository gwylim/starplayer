use std;
use std::hash::{Hash, Hasher};
use rand::{self, Rng};

use statistics::Stats;
use info::{BoardInfo, Pattern};
use player::Player;
use boardvec::BoardVec;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BoardState {
    pub moves: usize,
    pub first_player: BoardVec,
    pub second_player: BoardVec,
}

impl Hash for BoardState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first_player.hash(state);
        self.second_player.hash(state);
    }
}

fn dfs_points(info: &BoardInfo, player: BoardVec, this_player: bool, visited: &mut BoardVec, point: usize) -> usize {
    let mut result = 0;
    if info.on_boundary(point) {
        result += 1;
    }
    for other_point in info.adjacencies[point].iter() {
        if visited.get(*other_point) || this_player != player.get(*other_point) {
            continue;
        }
        visited.set(*other_point);
        result += dfs_points(info, player, this_player, visited, *other_point);
    }
    result
}

impl BoardState {
    pub fn new(info: &BoardInfo) -> BoardState {
        if info.count > BoardVec::size() {
            panic!("BoardVec does not support boards of this size");
        }
        BoardState {
            moves: 0,
            first_player: BoardVec::new(),
            second_player: BoardVec::new(),
        }
    }

    /**
     * The player who's turn it currently is
     */
    pub fn player_turn(&self) -> Player {
        if self.moves % 2 == 0 {
            Player::First
        } else {
            Player::Second
        }
    }

    /**
     * Sets the move of the current player to the given index, and increments number of moves by 1
     */
    pub fn add_move(&mut self, i: usize) {
        if self.moves%2 == 0 {
            self.first_player.set(i);
        } else {
            self.second_player.set(i);
        }
        self.moves += 1;
    }

    /**
     * Returns whether the board has been filled
     */
    pub fn finished(&self, info: &BoardInfo) -> bool {
        self.moves == info.count
    }

    /**
     * Returns the player's score, under the assumption that all hexes not occupied by them are
     * occupied by their opponent (thus giving their minimum possible score). If this is greater
     * than zero, then they have won.
     */
    pub fn player_score(&self, info: &BoardInfo, player: Player, komi: isize) -> isize {
        let player_points = match player {
            Player::First => self.first_player,
            Player::Second => self.second_player,
        };
        let starting_score = match player {
            Player::First => -komi,
            Player::Second => komi,
        };
        let mut visited = BoardVec::new();
        let mut player_score: isize = starting_score;
        for i in 0..info.count {
            if visited.get(i) {
                continue;
            }
            let this_player = player_points.get(i);
            visited.set(i);
            let score = dfs_points(info, player_points, this_player, &mut visited, i) as isize;
            let multiplier = if this_player { 1 } else { -1 };
            if score < 2 {
                player_score += (-1) * multiplier * score;
            } else {
                player_score += multiplier * (score - 4);
            }
        }
        player_score
    }

    /**
     * Returns whether the given player has won the game
     */
    pub fn is_winner(&self, info: &BoardInfo, player: Player, komi: isize) -> bool {
        self.player_score(info, player, komi) > 0
    }

    /**
     * Returns true if either player has played at the given position
     */
    pub fn any(&self, i: usize) -> bool {
        self.first_player.get(i) || self.second_player.get(i)
    }

    /**
     * Play random games starting from this game state, and return win statistics
     */
    pub fn play_random(self, info: &BoardInfo, komi: isize, iterations: u32) -> Stats {
        let mut result = Stats::new(info);
        for _ in 0..iterations {
            let mut new_state = self;
            new_state.play_random_inner(info);
            let winner = if new_state.is_winner(info, Player::First, komi) {
                Player::First
            } else {
                Player::Second
            };
            result.record_game(winner);
            for i in 0..info.count {
                result.record_point(winner, i, if new_state.first_player.get(i) { Player::First } else { Player::Second});
            }
        }
        result
    }

    /**
     * Print a representation of the current board to standard output
     */
    pub fn print_board(&self, info: &BoardInfo) {
        print!(" ");
        for x in 0..info.coords_range {
            print!(" {}", (x + 1) % 10);
        }
        println!("");
        for y in 0..info.coords_range {
            let padding = std::iter::repeat(" ").take(y + 1).collect::<String>();
            print!("{}{}", padding, (y + 1) % 10);
            for x in 0..info.coords_range {
                print!(" ");
                match info.reverse_coords.get(&(x, y)) {
                    None => {
                        print!("_");
                    },
                    Some(&idx) => {
                        if self.first_player.get(idx) {
                            if self.second_player.get(idx) {
                                panic!("Both players present at same board position");
                            }
                            print!("X");
                        } else if self.second_player.get(idx) {
                            print!("O");
                        } else {
                            print!(".");
                        }
                    },
                }
            }
            println!("");
        }
    }

    fn play_random_inner(&mut self, info: &BoardInfo) {
        let mut unplayed = Vec::with_capacity(info.count - self.moves);
        for i in 0..info.count {
            if !self.any(i) {
                unplayed.push(i);
            }
        }
        // TODO: initialize rng somewhere else
        rand::weak_rng().shuffle(&mut unplayed);
        let mut index = 0;
        let mut last_played = None;
        loop {
            if self.finished(info) {
                break;
            }

            let is_first_player = self.moves%2 == 0;

            let mut to_play = None;

            // Try playing pattern first
            if let Some(last_pos) = last_played {
                let patterns_for_pos: &Vec<Pattern> = &info.patterns[last_pos];
                for pattern in patterns_for_pos.iter() {
                    let (last_player, current_player) = if is_first_player {
                        (self.second_player, self.first_player)
                    } else {
                        (self.first_player, self.second_player)
                    };
                    if let Some(pos) = pattern.check(&last_player, &current_player) {
                        to_play = Some(pos);
                        break;
                    }
                }
            }

            // Play next empty position otherwise
            if let None = to_play {
                while index < unplayed.len() && self.any(unplayed[index]) {
                    index += 1;
                }
                if index < unplayed.len() {
                    to_play = Some(unplayed[index]);
                }
            }

            if let Some(pos) = to_play {
                if is_first_player {
                    self.first_player.set(pos);
                } else {
                    self.second_player.set(pos);
                }
                self.moves += 1;
                last_played = Some(pos);
            }
        }
    }
}
