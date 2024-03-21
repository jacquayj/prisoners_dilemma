mod strategies;

use strategies::History;
use strategies::Move;
use strategies::Payoff;
use strategies::Random;
use strategies::Strategy;
use strategies::TitForTat;

fn main() {
    let p1 = Player::new(Box::new(Random {}));
    let p2 = Player::new(Box::new(TitForTat {}));

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

struct PrisonerDilemmaGame {
    iterations: i32,
    history: History,
    p1: Player,
    p2: Player,
}

impl PrisonerDilemmaGame {
    fn new(p1: Player, p2: Player, iterations: i32) -> PrisonerDilemmaGame {
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

struct Player {
    score: i32,
    strategy: Box<dyn Strategy>,
}

impl Player {
    fn new(strat: Box<dyn Strategy>) -> Player {
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
