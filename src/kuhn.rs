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

pub struct ActionHistory(Vec<Action>);

impl ActionHistory {
    pub fn is_terminmal(&self) -> bool {
        return false;
    }
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
            action_history: ActionHistory(Vec::new()),
            players: Vec::new(),
        };
    }

    pub fn add_player(&mut self, mut p: Box<dyn player::Player>) {
        assert!(self.players.len() < 2);
        p.on_register(self.players.len() as i32);
        self.players.push(p);
    }

    pub fn start(&mut self, total_round: usize) {
        for _ in 0..total_round {
            while self.action_history.is_terminmal() == false {
                for (player_id, player) in self.players.iter_mut().enumerate() {
                    let action = player.decide_action(&self.action_history);
                    self.action_history.0.push(action);
                }
            }

            // game ends, calculate payoff
            let payoffs = self.get_payoff();

            for (player_id, player) in self.players.iter_mut().enumerate() {
                player.handle_result(&self.action_history, payoffs[player_id]);
            }
        }
    }

    // only 2 players
    fn get_payoff(&self) -> Vec<i64> {
        assert!(self.action_history.is_terminmal());
        assert!(self.cards[0] != self.cards[1]);

        if self.cards[0] > self.cards[1] {
            return vec![1, -1];
        }

        return vec![-1, 1];
    }
}
