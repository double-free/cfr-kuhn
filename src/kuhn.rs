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
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Action {
    Check,
    Bet,
}

impl Action {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        // total 2 actions
        let action_id = rng.gen_range(0..2);

        return Action::from_int(action_id);
    }

    pub fn from_int(action_id: usize) -> Self {
        let action = match action_id {
            0 => Action::Check,
            1 => Action::Bet,
            _ => panic!("unknown action id {}", action_id),
        };
        return action;
    }

    // 2 actions
    pub fn num() -> usize {
        return 2;
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct ActionHistory(pub Vec<Action>);

impl ActionHistory {
    pub fn new(raw: Vec<Action>) -> Self {
        return Self(raw);
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
        let mut rng = thread_rng();
        for round in 0..total_round {
            // game starts, shuffle card
            self.cards.shuffle(&mut rng);
            self.action_history.0.clear();
            println!("round {}, card {:?}", round, &self.cards);

            for (player_id, player) in self.players.iter_mut().enumerate() {
                player.on_start(self.cards[player_id]);
            }

            let mut maybe_payoff = get_payoff(&self.action_history, &self.cards);
            while maybe_payoff.is_none() {
                // not a terminal node, go on
                for player in self.players.iter_mut() {
                    let action = player.decide_action(&self.action_history);
                    self.action_history.0.push(action);

                    // recalculate after change
                    maybe_payoff = get_payoff(&self.action_history, &self.cards);
                    if maybe_payoff.is_some() {
                        break;
                    }
                }
            }

            let payoff = maybe_payoff.unwrap();
            let payoffs = vec![payoff, -payoff];
            for (player_id, player) in self.players.iter_mut().enumerate() {
                player.handle_result(&self.action_history, payoffs[player_id]);
            }
        }

        for (player_id, player) in self.players.iter().enumerate() {
            println!("player {}: {}", player_id, player.to_string());
        }
    }
}

// only 2 players, return the payoff of first player
// if this is not a terminal node, return None
pub fn get_payoff(action_history: &ActionHistory, cards: &Vec<i32>) -> Option<i64> {
    if action_history.0.len() < 2 {
        return None;
    }

    let prev_action = action_history.0[action_history.0.len() - 1];
    let prev_prev_action = action_history.0[action_history.0.len() - 2];

    // last action is a pass
    // pass->pass
    // pass->bet->pass
    // bet->pass
    if prev_action == Action::Check {
        if prev_prev_action == Action::Check {
            if cards[0] > cards[1] {
                return Some(1);
            }
            return Some(-1);
        }
        // the bet player
        let bet_player_id = action_history.0.len() % 2;
        if bet_player_id == 0 {
            return Some(1);
        }
        return Some(-1);
    }

    // last action is a bet
    // bet->bet
    // pass->bet->bet
    if prev_action == Action::Bet && prev_prev_action == Action::Bet {
        if cards[0] > cards[1] {
            return Some(2);
        }
        return Some(-2);
    }

    // pass->bet, not a terminal node
    return None;
}
