
fn main() {
    let mut p1 = TitForTat{score: 0};
    let mut p2 = TitForTat{score: 0};
    
    let mut hist: History = vec![];


    // Play 500 times
    for _ in 0..500 {
        play_round(&mut p1, &mut p2, &mut hist);
    }

    println!("Player 1 score: {}", p1.score());
    println!("Player 2 score: {}", p2.score());
}

#[derive(Clone, Debug)]
enum Move {
    Cooperate,
    Defect
}

type History = Vec<(Move, Move)>;

type Payoff = (i32, i32);

fn payoff(m1: Move, m2: Move) -> Payoff {
    match (m1, m2) {
        (Move::Cooperate, Move::Cooperate) => (2, 2),
        (Move::Cooperate, Move::Defect) => (0, 3),
        (Move::Defect, Move::Cooperate) => (3, 0),
        (Move::Defect, Move::Defect) => (1, 1)
    }
}

fn play_round(p1: &mut dyn Strategy, p2: &mut dyn Strategy, hist: &mut History) -> Payoff {
    let m1 = p1.play(hist.clone());
    let m2 = p2.play(hist.clone());
    let p = payoff(m1.clone(), m2.clone());
    
    p1.pay(p.0);
    p2.pay(p.1);

    hist.push((m1, m2));

    p
}

trait Strategy {
    fn play(&self, hist: History) -> Move;
    fn score(&self) -> i32;
    fn pay(&mut self, p: i32);
}

struct AlwaysCooperate {
    score: i32
}

impl Strategy for AlwaysCooperate {
    fn play(&self, _hist: History) -> Move {
        Move::Cooperate
    }

    fn score(&self) -> i32 {
        self.score
    }

    fn pay(&mut self, p: i32) {
        self.score += p;
    }
}

struct AlwaysDefect {
    score: i32
}

impl Strategy for AlwaysDefect {
    fn play(&self, _hist: History) -> Move {
        Move::Defect
    }

    fn score(&self) -> i32 {
        self.score
    }

    fn pay(&mut self, p: i32) {
        self.score += p;
    }
}

struct TitForTat {
    score: i32
}

impl Strategy for TitForTat {
    fn play(&self, hist: History) -> Move {
        match hist.last() {
            Some((_, m)) => m.clone(),
            None => Move::Cooperate
        }
    }

    fn score(&self) -> i32 {
        self.score
    }

    fn pay(&mut self, p: i32) {
        self.score += p;
    }
}
