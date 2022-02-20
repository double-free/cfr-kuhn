use crate::kuhn;
use crate::player::player;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::fmt;

// every information set has a corresponding node

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct InformationSet {
    action_history: kuhn::ActionHistory,
    hand_card: i32,
}

#[derive(Debug)]
struct CfrNode {
    cum_regrets: Vec<f64>,
}

impl CfrNode {
    pub fn new() -> Self {
        let node = CfrNode {
            cum_regrets: vec![0.0; kuhn::Action::num()],
        };
        return node;
    }

    pub fn get_action(&self) -> kuhn::Action {
        let mut regret_sum = 0;
        for regret in self.cum_regrets.iter() {
            regret_sum += *regret as i64;
        }

        if regret_sum <= 0 {
            // choose random
            return kuhn::Action::random();
        }

        // only contains positive regret actions
        let mut rng = thread_rng();
        let n: i64 = rng.gen_range(0..regret_sum);

        let mut s: i64 = 0;
        for (i, regret) in self.cum_regrets.iter().enumerate() {
            if regret <= &0.0 {
                continue;
            }

            s += *regret as i64;
            if s > n {
                return kuhn::Action::from_int(i as u32);
            }
        }

        panic!("never reach here");
    }

    pub fn get_action_probability(&self) -> Vec<f64> {
        let mut regret_sum = 0.0;
        for regret in self.cum_regrets.iter() {
            regret_sum += regret;
        }

        let mut result = vec![1.0 / kuhn::Action::num() as f64; kuhn::Action::num()];
        if regret_sum <= 0.0 {
            return result;
        }

        for i in 0..kuhn::Action::num() {
            result[i] = self.cum_regrets[i] / regret_sum;
        }

        return result;
    }
}

pub struct CfrPlayer {
    player_id: i32,
    hand_card: i32,
    money: i64,
    cfr_info: HashMap<InformationSet, CfrNode>,
}

impl CfrPlayer {
    pub fn new() -> Self {
        let player = CfrPlayer {
            player_id: -1,
            hand_card: -1,
            money: 0,
            cfr_info: HashMap::new(),
        };
        return player;
    }
}

impl fmt::Display for CfrPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "money = {}, cfr info = {:?}", self.money, self.cfr_info)
    }
}

impl player::Player for CfrPlayer {
    fn on_register(&mut self, player_id: i32) {
        self.player_id = player_id;
    }
    fn on_start(&mut self, card: i32) {
        self.hand_card = card;
    }
    fn decide_action(&mut self, action_history: &kuhn::ActionHistory) -> kuhn::Action {
        // no valid cfr action, use random
        let info_set = InformationSet {
            action_history: action_history.clone(),
            hand_card: self.hand_card,
        };
        if self.cfr_info.contains_key(&info_set) == false {
            self.cfr_info.insert(info_set, CfrNode::new());
            return kuhn::Action::random();
        }

        return self.cfr_info.get(&info_set).unwrap().get_action();
    }
    fn handle_result(&mut self, action_history: &kuhn::ActionHistory, payoff: i64) {
        self.money += payoff;
        let info_set = InformationSet {
            action_history: action_history.clone(),
            hand_card: self.hand_card,
        };
        println!(
            "player {} get payoff {} with info set {:?}",
            self.player_id, payoff, info_set
        )
    }
}

impl CfrPlayer {
    // play with self to reach Nash Equilibrium
    pub fn train(&mut self, iteration: usize) {
        let mut rng = thread_rng();
        for round in 0..iteration {
            let mut cards = vec![1, 2, 3];
            let action_history = kuhn::ActionHistory::new(Vec::new());
            // game starts, shuffle card
            cards.shuffle(&mut rng);

            println!("round {}, card {:?}", round, &cards);

            let mut history_probs = HashMap::from([(0, 1.0), (1, 1.0)]);
            self.cfr(action_history, &cards, &mut history_probs);
        }
    }

    fn cfr(
        &mut self,
        history: kuhn::ActionHistory,
        cards: &Vec<i32>,
        history_probs: &mut HashMap<i32, f64>,
    ) -> f64 {
        let player_id = history.0.len() % 2;

        let maybe_payoff = kuhn::get_payoff(&history, cards);
        if maybe_payoff.is_some() {
            let payoff = match player_id {
                0 => maybe_payoff.unwrap(),
                1 => -maybe_payoff.unwrap(),
                _ => panic!("unexpected player id {}", player_id),
            };
            return payoff as f64;
        }

        // not the terminal node
        let info_set = InformationSet {
            action_history: history.clone(),
            hand_card: cards[player_id as usize],
        };

        if self.cfr_info.contains_key(&info_set) == false {
            self.cfr_info.insert(info_set.clone(), CfrNode::new());
        }

        let action_probs = self
            .cfr_info
            .get(&info_set)
            .unwrap()
            .get_action_probability();
        let mut action_payoffs = Vec::new();
        let mut node_payoff = 0.0;
        for (action_id, prob) in action_probs.iter().enumerate() {
            // next history, appending the new action to it
            let mut next_history = history.clone();
            next_history
                .0
                .push(kuhn::Action::from_int(action_id as u32));
            // update history probability
            *history_probs.get_mut(&(player_id as i32)).unwrap() *= prob;

            // recursive call, "-" here because the return value is the opponent's payoff
            action_payoffs.push(-self.cfr(next_history, cards, history_probs));
            node_payoff += action_payoffs[action_id];
        }

        // update regret
        let node = self.cfr_info.get_mut(&info_set).unwrap();
        for (action_id, payoff) in action_payoffs.iter().enumerate() {
            let regret = payoff - node_payoff;
            node.cum_regrets[action_id] += action_probs[action_id] * regret;
        }

        return node_payoff;
    }
}
