use std::fs;

enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    fn from(s: &str) -> Moves {
        match s {
            "A" | "X" => Moves::Rock,
            "B" | "Y" => Moves::Paper,
            "C" | "Z" => Moves::Scissors,
            _ => panic!("Invalid encrypted input move"),
        }
    }

    fn from_result(opponent_move: &Moves, round_result: &RoundResult) -> Moves {
        match (opponent_move, round_result) {
            (Moves::Rock, RoundResult::Win)
            | (Moves::Paper, RoundResult::Draw)
            | (Moves::Scissors, RoundResult::Loss) => Moves::Paper,
            (Moves::Rock, RoundResult::Draw)
            | (Moves::Paper, RoundResult::Loss)
            | (Moves::Scissors, RoundResult::Win) => Moves::Rock,
            (Moves::Rock, RoundResult::Loss)
            | (Moves::Paper, RoundResult::Win)
            | (Moves::Scissors, RoundResult::Draw) => Moves::Scissors,
        }
    }
}

enum RoundResult {
    Win,
    Loss,
    Draw,
}

impl RoundResult {
    fn from(s: &str) -> RoundResult {
        match s {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => panic!("Invalid encrypted input move"),
        }
    }

    fn from_moves(player_move: &Moves, opponent_move: &Moves) -> RoundResult {
        match (player_move, opponent_move) {
            (Moves::Rock, Moves::Paper)
            | (Moves::Paper, Moves::Scissors)
            | (Moves::Scissors, Moves::Rock) => RoundResult::Loss,
            (Moves::Paper, Moves::Rock)
            | (Moves::Rock, Moves::Scissors)
            | (Moves::Scissors, Moves::Paper) => RoundResult::Win,
            _ => RoundResult::Draw,
        }
    }
}

fn main() {
    let file_path = "input.txt";
    let input_data = fs::read_to_string(file_path).unwrap();

    let score: i32 = input_data
        .split_terminator("\n")
        .map(|round| score_round1(round))
        .sum();

    println!("(1) The total score is [{score}]\n");

    let score: i32 = input_data
        .split_terminator("\n")
        .map(|round| score_round2(round))
        .sum();

    println!("(2) The total score is [{score}]");
}

fn score_round1(round: &str) -> i32 {
    let moves: Vec<Moves> = round.split(" ").map(|s| Moves::from(s)).collect();
    let opponent_move = &moves[0];
    let player_move = &moves[1];
    let round_score = match RoundResult::from_moves(player_move, opponent_move) {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Loss => 0,
    } + match player_move {
        Moves::Rock => 1,
        Moves::Paper => 2,
        Moves::Scissors => 3,
    };
    round_score
}

fn score_round2(round: &str) -> i32 {
    let round: Vec<&str> = round.split(" ").collect();
    let opponent_move = Moves::from(&round[0]);
    let round_result = RoundResult::from(&round[1]);
    let round_score = match round_result {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Loss => 0,
    } + match Moves::from_result(&opponent_move, &round_result) {
        Moves::Rock => 1,
        Moves::Paper => 2,
        Moves::Scissors => 3,
    };
    round_score
}
