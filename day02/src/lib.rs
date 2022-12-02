use std::fs;

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Tie,
}

// Read
// A Y
// B X
// C Y
// Where A, B, C are opponents moves
// Y, X, C is our move
fn read_input(file: &str) -> Vec<(Move, Move)> {
    let input: String = fs::read_to_string(file).expect("Error reading file");
    let input_vec: Vec<&str> = input.split('\n').collect();
    let mut moves: Vec<(Move, Move)> = Vec::new();
    for line in input_vec {
        let line_vec: Vec<&str> = line.split(' ').collect();
        let opponent_move = match line_vec[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid input"),
        };
        let our_move = match line_vec[1] {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Invalid input"),
        };
        moves.push((opponent_move, our_move));
    }
    moves
}

fn read_input_alt(file: &str) -> Vec<(Move, Result)> {
    let input: String = fs::read_to_string(file).expect("Error reading file");
    let input_vec: Vec<&str> = input.split('\n').collect();
    let mut moves_outcome: Vec<(Move, Result)> = Vec::new();
    for line in input_vec {
        let line_vec: Vec<&str> = line.split(' ').collect();
        let opponent_move = match line_vec[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid input"),
        };
        let our_move = match line_vec[1] {
            "X" => Result::Lose,
            "Y" => Result::Tie,
            "Z" => Result::Win,
            _ => panic!("Invalid input"),
        };
        moves_outcome.push((opponent_move, our_move));
    }
    moves_outcome
}

fn calc_score(moves: &Vec<(Move, Move)>) -> i32 {
    let mut score = 0;

    // let rock be worth 1
    // let paper be worth 2
    // let scissors be worth 3

    // our score is our move + 6 if we win, 3 if we tie, 0 if we lose
    for (opponent_move, our_move) in moves {
        let our_move_value = match our_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let outcome = match (opponent_move, our_move) {
            (Move::Rock, Move::Rock) => Result::Tie,
            (Move::Rock, Move::Paper) => Result::Win,
            (Move::Rock, Move::Scissors) => Result::Lose,
            (Move::Paper, Move::Rock) => Result::Lose,
            (Move::Paper, Move::Paper) => Result::Tie,
            (Move::Paper, Move::Scissors) => Result::Win,
            (Move::Scissors, Move::Rock) => Result::Win,
            (Move::Scissors, Move::Paper) => Result::Lose,
            (Move::Scissors, Move::Scissors) => Result::Tie,
        };

        score += match outcome {
            Result::Win => our_move_value + 6,
            Result::Tie => our_move_value + 3,
            Result::Lose => our_move_value,
        };
    }
    score
}

pub fn part1() {
    let moves = read_input("input/day02.in");
    let score = calc_score(&moves);
    println!("Part 1: {}", score);
}

pub fn part2() {
    let move_outcome = read_input_alt("input/day02.in");

    // determine our move based on the outcome
    let mut moves = Vec::new();
    for (opponent_move, outcome) in move_outcome {
        let our_move = match (opponent_move, outcome) {
            (Move::Rock, Result::Win) => Move::Paper,
            (Move::Rock, Result::Tie) => Move::Rock,
            (Move::Rock, Result::Lose) => Move::Scissors,
            (Move::Paper, Result::Win) => Move::Scissors,
            (Move::Paper, Result::Tie) => Move::Paper,
            (Move::Paper, Result::Lose) => Move::Rock,
            (Move::Scissors, Result::Win) => Move::Rock,
            (Move::Scissors, Result::Tie) => Move::Scissors,
            (Move::Scissors, Result::Lose) => Move::Paper,
        };
        moves.push((opponent_move, our_move));
    }

    let score = calc_score(&moves);

    println!("Part 2: {}", score);
}
