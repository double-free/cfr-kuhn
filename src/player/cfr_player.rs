use crate::kuhn;
use crate::player::player;
use std::collections::HashMap;

pub struct CfrPlayer {
    player_id: i32,
    hand_card: i32,
}

// returns the counterfactual value
fn cfr(
    history: &kuhn::ActionHistory,
    player_id: i32,
    reach_prob: &HashMap<i32, f64>,
    timestep: i64,
) -> i64 {
    return 0;
}

impl CfrPlayer {
    pub fn new() -> Self {
        let player = CfrPlayer{player_id:-1, hand_card:-1};
        return player;
    }
}

impl player::Player for CfrPlayer {
    fn on_register(&mut self, player_id: i32) {
        self.player_id = player_id;
    }
    fn on_start(&mut self, card: i32) {
        self.hand_card = card;
    }
    fn decide_action(&mut self, game_info: &kuhn::ActionHistory) -> kuhn::Action {
        // no valid cfr action, use random
        return kuhn::Action::random();
    }
    fn handle_result(&mut self, game_info: &kuhn::ActionHistory, payoff: i64) {}
}
