mod strategies;

use strategies::Payoff;
use strategies::History;
use strategies::Move;
use strategies::Strategy;
use strategies::TitForTat;

use strategies::Random;


fn main() {
    let mut p1 = Player::new(Box::new(TitForTat{}));
    let mut p2 = Player::new(Box::new(Random{}));

    let mut hist: History = Vec::new();

    for _ in 0..1000 {
        play_round(&mut p1, &mut p2, &mut hist);
    }

    println!("p1: {}, p2: {}", p1.score, p2.score);

}


fn payoff(m1: Move, m2: Move) -> Payoff {
    match (m1, m2) {
        (Move::Cooperate, Move::Cooperate) => (2, 2),
        (Move::Cooperate, Move::Defect) => (0, 3),
        (Move::Defect, Move::Cooperate) => (3, 0),
        (Move::Defect, Move::Defect) => (1, 1)
    }
}

fn play_round(p1: &mut Player, p2: &mut Player, hist: &mut History) {
    let m1 = p1.play(hist.clone());
    let m2 = p2.play(hist.clone());

    let (p1_pay, p2_pay) = payoff(m1.clone(), m2.clone());

    p1.pay(p1_pay);
    p2.pay(p2_pay);

    hist.push((m1, m2));
}

struct Player {
    score: i32,
    strategy: Box<dyn Strategy>
}

impl Player {
    fn new(strat: Box<dyn Strategy>) -> Player {
        Player{score: 0, strategy: strat}
    }

    fn play(&self, hist: History) -> Move {
        self.strategy.play(hist)
    }

    fn pay(&mut self, p: i32) {
        self.score += p;
    }
}

