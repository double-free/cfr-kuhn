// Kuhn Poker is a simple 3-card poker game by Harold E. Kuhn.
// Two players each ante 1 chip, i.e. bet 1 chip blind into the pot before the deal.
// Three cards, marked with numbers 1, 2, and 3, are shuffled,
// and one card is dealt to each player and held as private information.
// Play alternates starting with player 1.
// On a turn, a player may either pass or bet.
// A player that bets places an additional chip into the pot.
// When a player passes after a bet, the opponent takes all chips in the pot.
// When there are two successive passes or two successive bets,
// both players reveal their cards, and the player with the higher card takes all chips in the pot.

use crate::player::player;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Action {
    Check,
    Bet,
}

pub struct PlayerAction {
    player_id: i32,
    action: Action,
}

pub type ActionHistory = Vec<PlayerAction>;

struct KuhnNode {
    info_set: String,
    regret_sum: Vec<i64>,
}

pub struct KuhnGame {
    cards: Vec<i32>,

    action_history: ActionHistory,

    players: Vec<Box<dyn player::Player>>,
}

impl KuhnGame {
    pub fn new() -> Self {
        return Self {
            cards: vec![1, 2, 3],
            action_history: Vec::new(),
            players: Vec::new(),
        };
    }
}
