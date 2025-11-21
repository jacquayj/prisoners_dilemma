use clap::Parser;
use num_cpus;
use rand::Rng;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;

#[derive(Parser, Debug)]
#[command(name = "Prisoner's Dilemma Tournament")]
#[command(about = "Run a tournament of Prisoner's Dilemma strategies", long_about = None)]
struct Args {
    /// Number of threads to use (default: number of CPU cores)
    #[arg(short, long)]
    threads: Option<usize>,

    /// Number of iterations per game (default: 1000000)
    #[arg(short, long)]
    iterations: Option<i32>,

    /// Strategies to include in the tournament.
    /// Options: always-cooperate, always-defect, tit-for-tat, random, two-tits-for-tat
    /// Comma-separated list (default: all strategies)
    #[arg(short, long)]
    strategies: Option<String>,

    /// Verbose output showing additional tournament details
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let num_threads = args.threads.unwrap_or_else(num_cpus::get);
    let iterations = args.iterations.unwrap_or(1_000_000);

    // Parse strategy selection
    let strategies: Vec<Arc<dyn Strategy>> = if let Some(strat_str) = &args.strategies {
        parse_strategies(strat_str)
    } else {
        // Default: all strategies
        vec![
            Arc::new(AlwaysCooperate {}),
            Arc::new(AlwaysDefect {}),
            Arc::new(TitForTat {}),
            Arc::new(Random {}),
            Arc::new(TwoTitsForTat {}),
        ]
    };

    if args.verbose {
        eprintln!(
            "Running tournament with {} strategies, {} iterations, {} threads",
            strategies.len(),
            iterations,
            num_threads
        );
        eprintln!("Strategies: {}", strategies.iter().map(|s| s.name()).collect::<Vec<_>>().join(", "));
        eprintln!("Total games: {}\n", strategies.len() * strategies.len());
    }

    let pool = ThreadPool::new(num_threads);
    let (tx, rx) = channel::<(Player, Player)>();

    // play all strategies against each other
    for s1 in strategies.iter() {
        for s2 in strategies.iter() {
            let tx = tx.clone();
            let s1 = s1.clone();
            let s2 = s2.clone();

            // spawn a new thread to run the game
            pool.execute(move || {
                let p1 = Player::new(s1);
                let p2 = Player::new(s2);

                let mut game = PrisonerDilemmaGame::new(p1, p2, iterations);

                game.play();

                tx.send((game.p1, game.p2)).unwrap();
            });
        }
    }

    let mut scores: Vec<(Player, Player)> = Vec::new();

    // collect the results from all games
    for _ in 0..strategies.len() * strategies.len() {
        let (p1, p2) = rx.recv().unwrap();
        scores.push((p1, p2));
    }

    // print the results
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

fn parse_strategies(strategy_str: &str) -> Vec<Arc<dyn Strategy>> {
    let mut strategies: Vec<Arc<dyn Strategy>> = Vec::new();

    for strat in strategy_str.split(',') {
        match strat.trim().to_lowercase().as_str() {
            "always-cooperate" => strategies.push(Arc::new(AlwaysCooperate {})),
            "always-defect" => strategies.push(Arc::new(AlwaysDefect {})),
            "tit-for-tat" => strategies.push(Arc::new(TitForTat {})),
            "random" => strategies.push(Arc::new(Random {})),
            "two-tits-for-tat" => strategies.push(Arc::new(TwoTitsForTat {})),
            invalid => eprintln!("Warning: unknown strategy '{}', skipping", invalid),
        }
    }

    if strategies.is_empty() {
        eprintln!("No valid strategies specified, using all strategies");
        strategies = vec![
            Arc::new(AlwaysCooperate {}),
            Arc::new(AlwaysDefect {}),
            Arc::new(TitForTat {}),
            Arc::new(Random {}),
            Arc::new(TwoTitsForTat {}),
        ];
    }

    strategies
}

pub struct PrisonerDilemmaGame {
    pub iterations: i32,
    pub history: History,
    pub p1: Player,
    pub p2: Player,
}

impl PrisonerDilemmaGame {
    pub fn new(p1: Player, p2: Player, iterations: i32) -> PrisonerDilemmaGame {
        PrisonerDilemmaGame {
            p1,
            p2,
            iterations,
            history: Vec::new(),
        }
    }

    pub fn calculate_payoff(m1: &Move, m2: &Move) -> Payoff {
        match (m1, m2) {
            (Move::Cooperate, Move::Cooperate) => (2, 2),
            (Move::Cooperate, Move::Defect) => (0, 3),
            (Move::Defect, Move::Cooperate) => (3, 0),
            (Move::Defect, Move::Defect) => (1, 1),
        }
    }

    pub fn play(&mut self) {
        for _ in 0..self.iterations {
            self.play_round();
        }
    }

    pub fn play_round(&mut self) {
        let m1 = self.p1.play(&self.history, 0);
        let m2 = self.p2.play(&self.history, 1);

        let (p1_pay, p2_pay) = Self::calculate_payoff(&m1, &m2);

        self.p1.pay(p1_pay);
        self.p2.pay(p2_pay);

        self.history.push([m1, m2]);
    }
}

pub struct Player {
    pub score: i32,
    pub strategy: Arc<dyn Strategy>,
}

impl Player {
    pub fn new(strat: Arc<dyn Strategy>) -> Player {
        Player {
            score: 0,
            strategy: strat,
        }
    }

    pub fn play(&self, hist: &History, hist_inx: usize) -> Move {
        self.strategy.play(hist, hist_inx)
    }

    pub fn pay(&mut self, p: i32) {
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

#[cfg(test)]
mod tests {
    use super::*;

    // ============ PAYOFF TESTS ============

    #[test]
    fn test_payoff_both_cooperate() {
        let payoff = PrisonerDilemmaGame::calculate_payoff(&Move::Cooperate, &Move::Cooperate);
        assert_eq!(payoff, (2, 2));
    }

    #[test]
    fn test_payoff_p1_defect_p2_cooperate() {
        let payoff = PrisonerDilemmaGame::calculate_payoff(&Move::Defect, &Move::Cooperate);
        assert_eq!(payoff, (3, 0));
    }

    #[test]
    fn test_payoff_p1_cooperate_p2_defect() {
        let payoff = PrisonerDilemmaGame::calculate_payoff(&Move::Cooperate, &Move::Defect);
        assert_eq!(payoff, (0, 3));
    }

    #[test]
    fn test_payoff_both_defect() {
        let payoff = PrisonerDilemmaGame::calculate_payoff(&Move::Defect, &Move::Defect);
        assert_eq!(payoff, (1, 1));
    }

    // ============ PLAYER TESTS ============

    #[test]
    fn test_player_initialization() {
        let strategy = Arc::new(AlwaysCooperate {});
        let player = Player::new(strategy);
        assert_eq!(player.score, 0);
    }

    #[test]
    fn test_player_pay() {
        let strategy = Arc::new(AlwaysCooperate {});
        let mut player = Player::new(strategy);
        player.pay(5);
        assert_eq!(player.score, 5);
        player.pay(3);
        assert_eq!(player.score, 8);
    }

    // ============ STRATEGY TESTS ============

    #[test]
    fn test_always_cooperate_strategy() {
        let strategy = AlwaysCooperate {};
        let history: History = vec![];

        // Should cooperate on first move
        assert_eq!(strategy.play(&history, 0), Move::Cooperate);
        assert_eq!(strategy.play(&history, 1), Move::Cooperate);

        // Should cooperate after any history
        let history = vec![[Move::Defect, Move::Defect]];
        assert_eq!(strategy.play(&history, 0), Move::Cooperate);
    }

    #[test]
    fn test_always_cooperate_name() {
        let strategy = AlwaysCooperate {};
        assert_eq!(strategy.name(), "Always Cooperate");
    }

    #[test]
    fn test_always_defect_strategy() {
        let strategy = AlwaysDefect {};
        let history: History = vec![];

        // Should defect on first move
        assert_eq!(strategy.play(&history, 0), Move::Defect);
        assert_eq!(strategy.play(&history, 1), Move::Defect);

        // Should defect after any history
        let history = vec![[Move::Cooperate, Move::Cooperate]];
        assert_eq!(strategy.play(&history, 0), Move::Defect);
    }

    #[test]
    fn test_always_defect_name() {
        let strategy = AlwaysDefect {};
        assert_eq!(strategy.name(), "Always Defect");
    }

    #[test]
    fn test_tit_for_tat_first_move() {
        let strategy = TitForTat {};
        let history: History = vec![];

        // Should cooperate on first move
        assert_eq!(strategy.play(&history, 0), Move::Cooperate);
        assert_eq!(strategy.play(&history, 1), Move::Cooperate);
    }

    #[test]
    fn test_tit_for_tat_copies_opponent() {
        let strategy = TitForTat {};

        // Opponent cooperated last round
        let history = vec![[Move::Cooperate, Move::Cooperate]];
        assert_eq!(strategy.play(&history, 0), Move::Cooperate);
        assert_eq!(strategy.play(&history, 1), Move::Cooperate);

        // Opponent defected last round
        let history = vec![[Move::Defect, Move::Cooperate]];
        assert_eq!(strategy.play(&history, 0), Move::Cooperate); // p0 plays against p1's last move (Cooperate)
        assert_eq!(strategy.play(&history, 1), Move::Defect); // p1 plays against p0's last move (Defect)
    }

    #[test]
    fn test_tit_for_tat_name() {
        let strategy = TitForTat {};
        assert_eq!(strategy.name(), "TitForTat");
    }

    #[test]
    fn test_two_tits_for_tat_first_moves() {
        let strategy = TwoTitsForTat {};
        let history: History = vec![];

        // Should cooperate on first move (no history)
        assert_eq!(strategy.play(&history, 0), Move::Cooperate);

        // Should cooperate with one move in history
        let history = vec![[Move::Defect, Move::Defect]];
        assert_eq!(strategy.play(&history, 0), Move::Cooperate);
    }

    #[test]
    fn test_two_tits_for_tat_single_defection() {
        let strategy = TwoTitsForTat {};

        // Opponent defected only once
        let history = vec![[Move::Defect, Move::Cooperate]];
        assert_eq!(strategy.play(&history, 0), Move::Cooperate);
    }

    #[test]
    fn test_two_tits_for_tat_double_defection() {
        let strategy = TwoTitsForTat {};

        // Opponent defected twice
        let history = vec![
            [Move::Defect, Move::Cooperate],
            [Move::Defect, Move::Cooperate],
        ];
        assert_eq!(strategy.play(&history, 1), Move::Defect); // p1 should defect
    }

    #[test]
    fn test_two_tits_for_tat_alternating_defections() {
        let strategy = TwoTitsForTat {};

        // Opponent defected once, cooperated once
        let history = vec![
            [Move::Cooperate, Move::Defect],
            [Move::Defect, Move::Cooperate],
        ];
        assert_eq!(strategy.play(&history, 0), Move::Cooperate); // p0 should cooperate
    }

    #[test]
    fn test_two_tits_for_tat_name() {
        let strategy = TwoTitsForTat {};
        assert_eq!(strategy.name(), "TwoTitsForTat");
    }

    // ============ GAME MECHANICS TESTS ============

    #[test]
    fn test_game_initialization() {
        let p1 = Player::new(Arc::new(AlwaysCooperate {}));
        let p2 = Player::new(Arc::new(AlwaysDefect {}));
        let game = PrisonerDilemmaGame::new(p1, p2, 100);

        assert_eq!(game.iterations, 100);
        assert_eq!(game.history.len(), 0);
        assert_eq!(game.p1.score, 0);
        assert_eq!(game.p2.score, 0);
    }

    #[test]
    fn test_single_round() {
        let p1 = Player::new(Arc::new(AlwaysCooperate {}));
        let p2 = Player::new(Arc::new(AlwaysDefect {}));
        let mut game = PrisonerDilemmaGame::new(p1, p2, 1);

        game.play_round();

        // AlwaysCooperate vs AlwaysDefect: p1 gets 0, p2 gets 3
        assert_eq!(game.p1.score, 0);
        assert_eq!(game.p2.score, 3);
        assert_eq!(game.history.len(), 1);
        assert_eq!(game.history[0], [Move::Cooperate, Move::Defect]);
    }

    #[test]
    fn test_multiple_rounds() {
        let p1 = Player::new(Arc::new(AlwaysCooperate {}));
        let p2 = Player::new(Arc::new(AlwaysCooperate {}));
        let mut game = PrisonerDilemmaGame::new(p1, p2, 5);

        game.play();

        // Both cooperate every round: 5 * 2 = 10 each
        assert_eq!(game.p1.score, 10);
        assert_eq!(game.p2.score, 10);
        assert_eq!(game.history.len(), 5);
    }

    #[test]
    fn test_tit_for_tat_vs_always_cooperate() {
        let p1 = Player::new(Arc::new(TitForTat {}));
        let p2 = Player::new(Arc::new(AlwaysCooperate {}));
        let mut game = PrisonerDilemmaGame::new(p1, p2, 10);

        game.play();

        // Both should cooperate every round
        assert_eq!(game.p1.score, 20);
        assert_eq!(game.p2.score, 20);
    }

    #[test]
    fn test_tit_for_tat_vs_always_defect() {
        let p1 = Player::new(Arc::new(TitForTat {}));
        let p2 = Player::new(Arc::new(AlwaysDefect {}));
        let mut game = PrisonerDilemmaGame::new(p1, p2, 10);

        game.play();

        // First round: TitForTat cooperates, AlwaysDefect defects
        // p1 gets 0, p2 gets 3
        // Rounds 2-10: TitForTat defects, AlwaysDefect defects
        // p1 gets 1, p2 gets 1 each round
        // Total: p1 = 0 + 9 = 9, p2 = 3 + 9 = 12
        assert_eq!(game.p1.score, 9);
        assert_eq!(game.p2.score, 12);
    }

    #[test]
    fn test_always_defect_vs_always_defect() {
        let p1 = Player::new(Arc::new(AlwaysDefect {}));
        let p2 = Player::new(Arc::new(AlwaysDefect {}));
        let mut game = PrisonerDilemmaGame::new(p1, p2, 10);

        game.play();

        // Both defect every round: 10 * 1 = 10 each
        assert_eq!(game.p1.score, 10);
        assert_eq!(game.p2.score, 10);
    }

    // ============ STRATEGY PARSING TESTS ============

    #[test]
    fn test_parse_single_strategy() {
        let strategies = parse_strategies("always-cooperate");
        assert_eq!(strategies.len(), 1);
        assert_eq!(strategies[0].name(), "Always Cooperate");
    }

    #[test]
    fn test_parse_multiple_strategies() {
        let strategies = parse_strategies("always-cooperate,always-defect,tit-for-tat");
        assert_eq!(strategies.len(), 3);
        assert_eq!(strategies[0].name(), "Always Cooperate");
        assert_eq!(strategies[1].name(), "Always Defect");
        assert_eq!(strategies[2].name(), "TitForTat");
    }

    #[test]
    fn test_parse_strategies_with_whitespace() {
        let strategies = parse_strategies("always-cooperate, always-defect, tit-for-tat");
        assert_eq!(strategies.len(), 3);
    }

    #[test]
    fn test_parse_strategies_case_insensitive() {
        let strategies = parse_strategies("ALWAYS-COOPERATE,Always-Defect");
        assert_eq!(strategies.len(), 2);
    }

    #[test]
    fn test_parse_all_strategies() {
        let strategies = parse_strategies("always-cooperate,always-defect,tit-for-tat,random,two-tits-for-tat");
        assert_eq!(strategies.len(), 5);
    }

    #[test]
    fn test_parse_duplicate_strategies() {
        let strategies = parse_strategies("always-cooperate,always-cooperate");
        assert_eq!(strategies.len(), 2); // Duplicates are allowed
    }

    #[test]
    fn test_parse_invalid_strategy_falls_back() {
        let strategies = parse_strategies("invalid-strategy");
        // Should fall back to all strategies
        assert_eq!(strategies.len(), 5);
    }

    #[test]
    fn test_parse_mixed_valid_invalid() {
        let strategies = parse_strategies("always-cooperate,invalid");
        // Invalid strategy is skipped, uses the valid one
        assert_eq!(strategies.len(), 1);
        assert_eq!(strategies[0].name(), "Always Cooperate");
    }
}
