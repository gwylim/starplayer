extern crate rand;
extern crate fnv;

pub mod player;
mod boardvec;
mod info;
mod statistics;
mod board;

use fnv::FnvHashMap;
use rand::XorShiftRng;

use info::BoardInfo;
use board::BoardState;
use statistics::Stats;
use player::Player;

const AMAF_PARAMETER: f64 = 1000.;
const AMAF_LIMIT: f64 = 0.5;
const INNER_ITERATIONS: u32 = 32;

struct Node {
    pos: Option<usize>,
    children_created: bool,
    self_visits: u32,
    self_wins: u32,
    visits: u32,
    wins: u32,
}

impl Node {
    fn new() -> Node {
        Node {
            pos: None,
            children_created: false,
            self_visits: 0,
            self_wins: 0,
            visits: 0,
            wins: 0,
        }
    }

    fn with_pos(pos: usize) -> Node {
        Node {
            pos: Some(pos),
            children_created: false,
            self_visits: 0,
            self_wins: 0,
            visits: 0,
            wins: 0,
        }
    }

    fn winrate(&self) -> f64 {
        if self.visits == 0 {
            return 2.;
        }
        let amaf_winrate = (self.wins as f64) / (self.visits as f64);
        let self_winrate = if self.self_visits == 0 {
            amaf_winrate
        } else {
            (self.self_wins as f64) / (self.self_visits as f64)
        };
        let alpha = AMAF_LIMIT * self.self_visits as f64 / (AMAF_PARAMETER + self.self_visits as f64);
        alpha * self_winrate + (1. - alpha) * amaf_winrate
    }
}

fn create_children(info: &BoardInfo, table: &mut FnvHashMap<BoardState, Node>, state: BoardState) {
    for i in 0..info.count {
        if !state.any(i) {
            let mut child_state = state;
            child_state.add_move(i);
            if !table.contains_key(&child_state) {
                table.insert(child_state, Node::with_pos(i));
            }
        }
    }
    table.get_mut(&state).unwrap().children_created = true;
}

fn play(
    info: &BoardInfo,
    rng: &mut XorShiftRng,
    table: &mut FnvHashMap<BoardState, Node>,
    state: BoardState,
    komi: isize,
) -> Stats {
    if state.finished(&info) {
        // TODO: move this logic to BoardState
        let winner = if state.is_winner(info, Player::First, komi) {
            Player::First
        } else {
            Player::Second
        };
        return Stats::single(info, winner, INNER_ITERATIONS);
    }
    let (node_self_visits, node_children_created) = {
        let node = table.get(&state).unwrap();
        (node.self_visits, node.children_created)
    };
    let stats = if node_self_visits == 0 {
        state.play_random(info, rng, komi, INNER_ITERATIONS)
    } else {
        if !node_children_created {
            create_children(info, table, state);
        }
        let mut max_winrate = -1.;
        let mut best_child_state = None;
        for i in 0..info.count {
            if !state.any(i) {
                let mut child_state = state;
                child_state.add_move(i);
                let child = table.get(&child_state).unwrap();
                let winrate = child.winrate();
                if winrate > max_winrate {
                    max_winrate = winrate;
                    best_child_state = Some(child_state);
                }
            }
        }
        play(info, rng, table, best_child_state.unwrap(), komi)
    };
    update(info, table, state, &stats);
    stats
}

fn update(
    info: &BoardInfo,
    table: &mut FnvHashMap<BoardState, Node>,
    state: BoardState,
    stats: &Stats,
) {
    let is_first_player = state.moves%2 != 0;
    let children_created = {
        if !table.contains_key(&state) {
            println!("Bad key {:?}", state);
        }
        let node = table.get_mut(&state).unwrap();
        node.self_visits += stats.count;
        if is_first_player {
            node.self_wins += stats.first_player_wins;
        } else {
            node.self_wins += stats.count - stats.first_player_wins;
        }
        node.children_created
    };
    if !children_created {
        return;
    }
    for i in 0..info.count {
        if !state.any(i) {
            let mut child_state = state;
            child_state.add_move(i);
            let child = table.get_mut(&child_state).unwrap();
            if !is_first_player {
                child.visits += stats.point_stats[i].p1;
                child.wins += stats.point_stats[i].p1_wins;
            } else {
                child.visits += stats.point_stats[i].p2;
                child.wins += stats.point_stats[i].p2_wins;
            }
        }
    }
}

fn best_move(
    info: &BoardInfo,
    table: &FnvHashMap<BoardState, Node>,
    state: BoardState,
) -> usize {
    let mut most_visits = 0;
    let mut best_move = None;
    for i in 0..info.count {
        if !state.any(i) {
            let mut child_state = state;
            child_state.add_move(i);
            let child = table.get(&child_state).unwrap();
            if let None = best_move {
                most_visits = child.self_visits;
                best_move = Some(child.pos);
            } else if child.self_visits > most_visits {
                most_visits = child.self_visits;
                best_move = Some(child.pos);
            }
        }
    }
    best_move.unwrap().unwrap()
}

pub struct StarAI {
    info: BoardInfo,
    state: BoardState,
    table: FnvHashMap<BoardState, Node>,
    rng: XorShiftRng,
}

// TODO: move implementations into here
// TODO: pass komi in on construction
// TODO: rename to something else
impl StarAI {
    pub fn new(size: usize) -> StarAI {
        let info = BoardInfo::new(size);
        let state = BoardState::new(&info);
        let mut table = FnvHashMap::default();
        table.insert(state, Node::new());
        StarAI {
            info: info,
            state: state,
            table: table,
            rng: rand::weak_rng(),
        }
    }

    // TODO: figure out how to thread komi through everything
    pub fn calculate(&mut self, iterations: usize, komi: isize) {
        for _ in 0..iterations {
            play(&self.info, &mut self.rng, &mut self.table, self.state, komi);
        }
    }

    pub fn best_move(&self) -> (usize, usize) {
        let index = best_move(&self.info, &self.table, self.state);
        self.info.coords[index]
    }

    pub fn add_move(&mut self, x: usize, y: usize) {
        self.state.add_move(*self.info.reverse_coords.get(&(x, y)).unwrap());
        self.table = FnvHashMap::default();
        self.table.insert(self.state, Node::new());
    }

    pub fn size(&self) -> usize {
        self.info.size
    }

    pub fn print_board(&self) {
        self.state.print_board(&self.info);
    }

    pub fn winner(&self, komi: isize) -> Option<Player> {
        for player in vec![Player::First, Player::Second] {
            if self.state.player_score(&self.info, player, komi) > 0 {
                return Some(player);
            }
        }
        None
    }

    pub fn score(&self, player: Player, komi: isize) -> isize {
        self.state.player_score(&self.info, player, komi)
    }

    pub fn finished(&self, komi: isize) -> bool {
        self.winner(komi).is_some()
    }

    pub fn player_turn(&self) -> Player {
        self.state.player_turn()
    }
}
