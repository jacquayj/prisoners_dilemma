use rand::Rng;

#[derive(Clone, Debug)]
pub enum Move {
    Cooperate,
    Defect
}

pub type History = Vec<(Move, Move)>;

pub type Payoff = (i32, i32);

pub trait Strategy {
    fn play(&self, hist: History) -> Move;
}

pub struct AlwaysCooperate;

impl Strategy for AlwaysCooperate {
    fn play(&self, _hist: History) -> Move {
        Move::Cooperate
    }
}

pub struct AlwaysDefect;

impl Strategy for AlwaysDefect {
    fn play(&self, _hist: History) -> Move {
        Move::Defect
    }
}

pub struct TitForTat;

impl Strategy for TitForTat {
    fn play(&self, hist: History) -> Move {
        match hist.last() {
            Some((_, m)) => m.clone(),
            None => Move::Cooperate
        }
    }
}

pub struct Random;

impl Strategy for Random {
    fn play(&self, _hist: History) -> Move {
        if rand::thread_rng().gen_bool(0.5) {
            Move::Cooperate
        } else {
            Move::Defect
        }
    }
}