use crate::kuhn;
use crate::player::player;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fmt;

// returns the counterfactual value
fn cfr(
    history: &kuhn::ActionHistory,
    player_id: i32,
    reach_prob: &HashMap<i32, f64>,
    timestep: i64,
) -> i64 {
    return 0;
}

// every information set has a corresponding node

#[derive(Hash, PartialEq, Eq, Debug)]
struct InformationSet {
    action_history: kuhn::ActionHistory,
    hand_card: i32,
}

#[derive(Debug)]
struct CfrNode {
    cum_regrets: Vec<i64>,
}

impl CfrNode {
    pub fn new() -> Self {
        let node = CfrNode {
            cum_regrets: vec![0; kuhn::Action::num()],
        };
        return node;
    }

    pub fn get_action(&self) -> kuhn::Action {
        let mut regret_sum = 0;
        for regret in self.cum_regrets.iter() {
            regret_sum += regret;
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
            if regret <= &0 {
                continue;
            }

            s += regret;
            if s > n {
                return kuhn::Action::from_int(i as u32);
            }
        }

        panic!("never reach here");
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
