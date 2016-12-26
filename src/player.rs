#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    First,
    Second,
}

pub const PLAYERS: [Player; 2] = [Player::First, Player::Second];
