use num_cpus;
use rand::Rng;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;

fn main() {

    // vector of strategies to play against each other
    let strategies: Vec<Arc<dyn Strategy>> = vec![
        // Arc is used to share ownership of the strategy objects between the various players and games
        // and to ensure that the strategies are thread-safe
        Arc::new(AlwaysCooperate {}),
        Arc::new(AlwaysDefect {}),
        Arc::new(TitForTat {}),
        Arc::new(Random {}),
        Arc::new(TwoTitsForTat {}),
    ];

    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel::<(Player, Player)>();

    // play all strategies against each other
    for s1 in strategies.iter() {
        for s2 in strategies.iter() {
            let tx = tx.clone();
            let s1 = s1.clone();
            let s2 = s2.clone();

            pool.execute(move || {
                let  p1 = Player::new(s1);
                let  p2 = Player::new(s2);

                let mut game = PrisonerDilemmaGame::new(p1, p2, 1000000);
                
                game.play();

                tx.send((game.p1, game.p2)).unwrap();
            });
        }
    }

    let mut scores: Vec<(Player, Player)> = Vec::new();

    for _ in 0..strategies.len() * strategies.len() {
        let (p1, p2) = rx.recv().unwrap();
        scores.push((p1, p2));
    }

    for (p1, p2) in scores {
        println!(
            "{} vs {}: {} vs {}",
            p1.strategy.name(),
            p2.strategy.name(),
            p1.score,
            p2.score
        );
    }
   
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
    strategy: Arc<dyn Strategy>,
}

impl Player {
    fn new(strat: Arc<dyn Strategy>) -> Player {
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

pub trait Strategy: Send + Sync {
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
