use crate::kuhn;

pub trait Player {
    fn on_register(&mut self, player_id: i32);
    fn on_start(&mut self, card: i32);
    fn decide_action(&mut self, game_info: &kuhn::ActionHistory) -> kuhn::Action;
    fn handle_result(&mut self, game_info: &kuhn::ActionHistory, payoff: i64);
}
