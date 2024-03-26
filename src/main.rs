use rand::Rng;


fn main() {
    let p1 = Player::new(Random {});
    let p2 = Player::new(TitForTat {});

    let mut game = PrisonerDilemmaGame::new(p1, p2, 1000000);

    game.play();

    println!(
        "Player 1 score ({}); {}",
        game.p1.score,
        game.p1.strategy.name()
    );
    println!(
        "Player 2 score ({}); {}",
        game.p2.score,
        game.p2.strategy.name()
    );
}

struct PrisonerDilemmaGame<S1: Strategy, S2: Strategy> {
    iterations: i32,
    history: History,
    p1: Player<S1>,
    p2: Player<S2>,
}

impl<S1: Strategy, S2: Strategy> PrisonerDilemmaGame<S1, S2> {
    fn new(p1: Player<S1>, p2: Player<S2>, iterations: i32) -> PrisonerDilemmaGame<S1, S2>{
        PrisonerDilemmaGame {
            p1,
            p2,
            iterations,
            history: Vec::new(),
        }
    }

    fn calculate_payoff(m1: &Move, m2: &Move) -> Payoff {
        match (m1, m2) {
            (Move::Cooperate, Move::Cooperate) => (2, 2),
            (Move::Cooperate, Move::Defect) => (0, 3),
            (Move::Defect, Move::Cooperate) => (3, 0),
            (Move::Defect, Move::Defect) => (1, 1),
        }
    }

    fn play(&mut self) {
        for _ in 0..self.iterations {
            self.play_round();
        }
    }

    fn play_round(&mut self) {
        let m1 = self.p1.play(&self.history, 0);
        let m2 = self.p2.play(&self.history, 1);

        let (p1_pay, p2_pay) = Self::calculate_payoff(&m1, &m2);

        self.p1.pay(p1_pay);
        self.p2.pay(p2_pay);

        self.history.push([m1, m2]);
    }
}

struct Player<S: Strategy> {
    score: i32,
    strategy: S,
}

impl<S: Strategy> Player<S> {
    fn new(strat: S) -> Player<S> {
        Player {
            score: 0,
            strategy: strat,
        }
    }

    fn play(&self, hist: &History, hist_inx: usize) -> Move {
        self.strategy.play(hist, hist_inx)
    }

    fn pay(&mut self, p: i32) {
        self.score += p;
    }
}


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
