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

impl fmt::Display for InformationSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{:?}", self.hand_card, self.action_history.0)
    }
}

#[derive(Debug)]
struct CfrNode {
    // this is used while training to get action
    cum_regrets: Vec<f64>,
    // this is the final output, it converges to Nash Equilibrium
    cum_strategy: Vec<f64>,
    // epsilon is the probability to "explore" actions
    epsilon: f64,
}

fn sample(probs: &Vec<f64>) -> usize {
    let mut rng = thread_rng();
    let thresh: f64 = rng.gen_range(0.0..1.0);
    let mut result: usize = 0;
    let mut cum = 0.0;
    for (i, prob) in probs.iter().enumerate() {
        cum += prob;
        if cum > thresh {
            result = i;
            break;
        }
    }

    return result;
}

impl CfrNode {
    pub fn new(epsilon: f64) -> Self {
        let node = CfrNode {
            cum_regrets: vec![0.0; kuhn::Action::num()],
            cum_strategy: vec![0.0; kuhn::Action::num()],
            epsilon: epsilon,
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
                return kuhn::Action::from_int(i);
            }
        }

        panic!("never reach here");
    }

    pub fn get_action_probability(&self) -> Vec<f64> {
        let mut regret_sum = 0.0;
        for regret in self.cum_regrets.iter() {
            if *regret > 0.0 {
                regret_sum += regret;
            }
        }

        let mut result = vec![1.0 / kuhn::Action::num() as f64; kuhn::Action::num()];
        if regret_sum <= 0.0 {
            return result;
        }

        for i in 0..kuhn::Action::num() {
            if self.cum_regrets[i] > 0.0 {
                result[i] = self.cum_regrets[i] / regret_sum;
            } else {
                result[i] = 0.0;
            }
        }

        // explore other actions with probability = epsilon
        for prob in result.iter_mut() {
            let explore_prob = self.epsilon / (kuhn::Action::num() as f64);
            *prob = explore_prob + (*prob) * (1.0 - self.epsilon);
        }

        return result;
    }
}

impl fmt::Display for CfrNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let check_ratio = self.cum_strategy[0] / (self.cum_strategy[0] + self.cum_strategy[1]);
        let bet_ratio = 1.0 - check_ratio;
        write!(f, "[Check: {}, Bet: {}]", check_ratio, bet_ratio)
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
        let mut ok = writeln!(f, "money = {}, cfr info = ", self.money);
        for (info_set, node) in self.cfr_info.iter() {
            ok = writeln!(f, "    {} - {}", info_set, node);
        }
        ok = writeln!(f, "details:");
        for (info_set, node) in self.cfr_info.iter() {
            ok = writeln!(f, "    {} - {:?}", info_set, node);
        }
        return ok;
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
            self.cfr_info.insert(info_set, CfrNode::new(0.0));
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
        let mut total_payoff = 0.0;
        for round in 0..iteration {
            let mut cards = vec![1, 2, 3];
            let action_history = kuhn::ActionHistory::new(Vec::new());
            // game starts, shuffle card
            cards.shuffle(&mut rng);
            let history_probs = HashMap::from([(0, 1.0), (1, 1.0)]);
            total_payoff += self.mccfr(action_history, &cards, history_probs);
        }

        println!(
            "round {}, average payoff {}",
            iteration,
            total_payoff / iteration as f64
        );
    }

    fn cfr(
        &mut self,
        history: kuhn::ActionHistory,
        cards: &Vec<i32>,
        reach_probs: HashMap<i32, f64>,
    ) -> f64 {
        // current active player
        let player_id = (history.0.len() % 2) as i32;

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
            self.cfr_info.insert(info_set.clone(), CfrNode::new(0.0));
        }

        let action_probs = self
            .cfr_info
            .get(&info_set)
            .unwrap()
            .get_action_probability();

        let mut action_payoffs = Vec::new();
        let mut node_value = 0.0;
        for (action_id, action_prob) in action_probs.iter().enumerate() {
            // next history, appending the new action to it
            let mut next_history = history.clone();
            next_history.0.push(kuhn::Action::from_int(action_id));
            // update history probability
            let mut next_reach_probs = reach_probs.clone();
            *next_reach_probs.get_mut(&player_id).unwrap() *= action_prob;

            // recursive call, "-" here because the return value is the opponent's payoff
            action_payoffs.push(-self.cfr(next_history, cards, next_reach_probs));
            node_value += action_prob * action_payoffs[action_id];
        }

        assert_eq!(action_payoffs.len(), 2);

        // update regret
        let node = self.cfr_info.get_mut(&info_set).unwrap();
        for (action_id, payoff) in action_payoffs.iter().enumerate() {
            let regret = payoff - node_value;
            let opponent = 1 - player_id;
            node.cum_regrets[action_id] += reach_probs[&opponent] * regret;
        }

        for (action_id, action_prob) in action_probs.iter().enumerate() {
            node.cum_strategy[action_id] += reach_probs[&player_id] * action_prob;
        }

        return node_value;
    }

    fn mccfr(
        &mut self,
        history: kuhn::ActionHistory,
        cards: &Vec<i32>,
        reach_probs: HashMap<i32, f64>,
    ) -> f64 {
        // current active player
        let player_id = (history.0.len() % 2) as i32;
        let opponent_id = 1 - player_id;

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
            self.cfr_info.insert(info_set.clone(), CfrNode::new(0.06));
        }

        let action_probs = self
            .cfr_info
            .get(&info_set)
            .unwrap()
            .get_action_probability();

        let chosen_action_id = sample(&action_probs);
        let chosen_action = kuhn::Action::from_int(chosen_action_id);
        let chosen_action_prob = action_probs[chosen_action_id as usize];
        let mut next_history = history.clone();
        next_history.0.push(chosen_action);
        // modify reach prob for SELF (not opponent)
        // update history probability
        let mut next_reach_probs = reach_probs.clone();
        *next_reach_probs.get_mut(&player_id).unwrap() *= chosen_action_prob;
        // recursive call
        // final payoff of the terminal node
        let final_payoff = -self.mccfr(next_history, cards, next_reach_probs);

        // update regret value
        let node = self.cfr_info.get_mut(&info_set).unwrap();
        for (action_id, action_prob) in action_probs.iter().enumerate() {
            let action = kuhn::Action::from_int(action_id);
            // reach probability of SELF (not opponent)
            let weight = final_payoff / reach_probs[&player_id] / action_prob;
            if action == chosen_action {
                node.cum_regrets[action_id] += weight * (1.0 - action_prob);
            } else {
                node.cum_regrets[action_id] += -weight * action_prob;
            }
        }

        // update strategy
        for (action_id, action_prob) in action_probs.iter().enumerate() {
            node.cum_strategy[action_id] += action_prob * reach_probs[&player_id];
        }

        return final_payoff;
    }
}
