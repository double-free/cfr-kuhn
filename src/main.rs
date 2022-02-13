mod kuhn;
mod player;

fn main() {
    let mut game = kuhn::KuhnGame::new();
    let player1 = player::cfr_player::CfrPlayer::new();
    let player2 = player::cfr_player::CfrPlayer::new();
    game.add_player(Box::new(player1));
    game.add_player(Box::new(player2));
    game.start(1000);
}
