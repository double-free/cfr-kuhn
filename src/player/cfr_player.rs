use crate::kuhn;
use crate::player::player;

struct CfrPlayer {
    player_id: i32,
}

impl player::Player for CfrPlayer {
    fn on_register(&mut self, player_id: i32) {
        self.player_id = player_id;
    }
    fn decide_action(&mut self, game_info: &kuhn::ActionHistory) -> kuhn::Action {
        return kuhn::Action::Check;
    }
    fn handle_result(&mut self, game_info: &kuhn::ActionHistory, payoff: i64) {}
}
