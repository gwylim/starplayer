use info::BoardInfo;
use player::Player;

/**
 * Stores the win statistics for a set of playouts
 */
pub struct Stats {
    /// Number of playouts
    pub count: u32,
    pub first_player_wins: u32,
    /// Win statistics grouped by the player at each point on the board
    pub point_stats: Vec<PointStats>,
}

impl Stats {
    pub fn new(info: &BoardInfo) -> Stats {
        let mut points = Vec::with_capacity(info.count as usize);
        for _ in 0..info.count {
            points.push(PointStats::new());
        }
        Stats {
            count: 0,
            first_player_wins: 0,
            point_stats: points,
        }
    }

    /**
     * Return win statistics as if the first player won `count` times. This is used for nodes where
     * the entire board has already been filled, so the winner is always determined.
     */
    pub fn single(info: &BoardInfo, winner: Player, count: u32) -> Stats {
        let mut points = Vec::with_capacity(info.count as usize);
        for _ in 0..info.count {
            points.push(PointStats::new());
        }
        let first_player_wins = match winner {
            Player::First => count,
            Player::Second => 0,
        };
        Stats {
            count: count,
            first_player_wins: first_player_wins,
            point_stats: points,
        }
    }

    /**
     * Record a game along with the winner
     */
    pub fn record_game(&mut self, winner: Player) {
        self.count += 1;
        if winner == Player::First {
            self.first_player_wins += 1;
        }
    }

    /**
     * Record the player who played at a particular point in a game, as well as whether they won
     */
    pub fn record_point(&mut self, winner: Player, point: usize, player: Player) {
        match player {
            Player::First => {
                self.point_stats[point].p1 += 1;
                if winner == player {
                    self.point_stats[point].p1_wins += 1;
                }
            },
            Player::Second => {
                self.point_stats[point].p2 += 1;
                if winner == player {
                    self.point_stats[point].p2_wins += 1;
                }
            },
        }
    }
}

/**
 * Represents the win statistics for a particular point. For each point, we count how in how many
 * playouts each player played there, and in how many playouts each player won in each case.
 */
// TODO: refactor out (p1, p1_wins) and (p2, p2_wins) as a different struct
#[derive(Copy, Clone)]
pub struct PointStats {
    /// Number of times first player played here
    pub p1: u32,
    /// Number of times first player won when they played here
    pub p1_wins: u32,
    /// Number of times second player played here
    pub p2: u32,
    /// Number of times second player won when they played here
    pub p2_wins: u32,
}

impl PointStats {
    pub fn new() -> PointStats {
        PointStats {
            p1: 0,
            p1_wins: 0,
            p2: 0,
            p2_wins: 0,
        }
    }
}
