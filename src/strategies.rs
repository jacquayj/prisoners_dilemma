use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum Move {
    Cooperate,
    Defect,
}

pub type History = Vec<[Move; 2]>;

pub type Payoff = (i32, i32);

pub trait Strategy {
    fn play(&self, hist: &History, hist_inx: usize) -> Move;
    fn name(&self) -> String;
}

pub struct AlwaysCooperate;

impl Strategy for AlwaysCooperate {
    fn play(&self, _hist: &History, _inx: usize) -> Move {
        Move::Cooperate
    }
    fn name(&self) -> String {
        "Always Cooperate".to_string()
    }
}

pub struct AlwaysDefect;

impl Strategy for AlwaysDefect {
    fn play(&self, _hist: &History, _inx: usize) -> Move {
        Move::Defect
    }
    fn name(&self) -> String {
        "Always Defect".to_string()
    }
}

pub struct TitForTat;

impl Strategy for TitForTat {
    fn play(&self, hist: &History, inx: usize) -> Move {
        // get the opponent's index
        let opponent_inx = if inx == 1 { 0 } else { 1 };

        // if there are no rounds, cooperate
        // otherwise, play the opponent's last move
        match hist.last() {
            Some(round) => round[opponent_inx].clone(),
            None => Move::Cooperate,
        }
    }
    fn name(&self) -> String {
        "TitForTat".to_string()
    }
}

pub struct Random;

impl Strategy for Random {
    fn play(&self, _hist: &History, _inx: usize) -> Move {
        if rand::thread_rng().gen_bool(0.5) {
            Move::Cooperate
        } else {
            Move::Defect
        }
    }
    fn name(&self) -> String {
        "Random".to_string()
    }
}

pub struct TwoTitsForTat;

impl Strategy for TwoTitsForTat {
    fn play(&self, hist: &History, inx: usize) -> Move {
        // get the opponent's index
        let opponent_inx = if inx == 1 { 0 } else { 1 };

        // get last two moves
        let last_two = hist.iter().rev().take(2).collect::<Vec<_>>();

        // if there are less than two moves, cooperate
        // otherwise, if the opponent defected in the last two rounds, defect
        match last_two.as_slice() {
            [round1, round2] => {
                if round1[opponent_inx] == Move::Defect && round2[opponent_inx] == Move::Defect {
                    Move::Defect
                } else {
                    Move::Cooperate
                }
            }
            _ => Move::Cooperate,
        }
    }
    fn name(&self) -> String {
        "TwoTitsForTat".to_string()
    }
}
