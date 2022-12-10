use std::error;

use crate::utils::read_file;
#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

pub(crate) fn run_aoc() -> Result<(i32, i32), Box<dyn error::Error>> {
    let lines = read_file("resource/aoc2/input.txt")?;
    let results = process_lines(lines.clone());
    let results2 = process_lines_v2(lines);
    Ok((results, results2))
}

fn process_lines(lines: Vec<String>) -> i32 {
    let mut total_results = 0;
    for line in lines {
        let result = process_line(line);
        total_results += result;
    }
    total_results
}

fn process_line(line: String) -> i32 {
    let char_vec = process_char_vec(line);

    let opponent_char = char_vec[0];
    let opponent_move = process_letter_as_move(opponent_char);

    let my_char = char_vec[2];
    let my_move = process_letter_as_move(my_char);
    let move_value = determine_move_value(my_move);

    let outcome = determine_outcome(opponent_move, my_move);
    let outcome_value = determine_outcome_value(outcome);

    move_value + outcome_value
}

fn process_lines_v2(lines: Vec<String>) -> i32 {
    let mut total_results = 0;
    for line in lines {
        let result = process_line_v2(line);
        total_results += result;
    }
    total_results
}

fn process_line_v2(line: String) -> i32 {
    let char_vec = process_char_vec(line);

    let opponent_char = char_vec[0];
    let opponent_move = process_letter_as_move(opponent_char);

    let my_char = char_vec[2];
    let my_outcome = process_letter_as_outcome(my_char);
    let outcome_value = determine_outcome_value(my_outcome);
    let my_move = determine_move_from_outcome(opponent_move, my_outcome);
    let move_value = determine_move_value(my_move);

    move_value + outcome_value
}

fn process_char_vec(line: String) -> Vec<char> {
    let mut char_vec = vec![];
    let chars = line.chars();
    for c in chars {
        char_vec.push(c);
    }
    char_vec
}

fn determine_outcome(opponent: RPS, me: RPS) -> Outcome {
    match (opponent, me) {
        (RPS::Rock, RPS::Rock) => Outcome::Draw,
        (RPS::Rock, RPS::Paper) => Outcome::Win,
        (RPS::Rock, RPS::Scissors) => Outcome::Lose,
        (RPS::Paper, RPS::Rock) => Outcome::Lose,
        (RPS::Paper, RPS::Paper) => Outcome::Draw,
        (RPS::Paper, RPS::Scissors) => Outcome::Win,
        (RPS::Scissors, RPS::Rock) => Outcome::Win,
        (RPS::Scissors, RPS::Paper) => Outcome::Lose,
        (RPS::Scissors, RPS::Scissors) => Outcome::Draw,
    }
}

fn process_letter_as_move(letter: char) -> RPS {
    match letter {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => RPS::Rock,
    }
}

fn process_letter_as_outcome(letter: char) -> Outcome {
    match letter {
        'Z' => Outcome::Win,
        'Y' => Outcome::Draw,
        'X' => Outcome::Lose,
        _ => Outcome::Win,
    }
}

fn determine_move_from_outcome(opponent: RPS, outcome: Outcome) -> RPS {
    match (opponent, outcome) {
        (RPS::Rock, Outcome::Win) => RPS::Paper,
        (RPS::Rock, Outcome::Draw) => RPS::Rock,
        (RPS::Rock, Outcome::Lose) => RPS::Scissors,
        (RPS::Paper, Outcome::Win) => RPS::Scissors,
        (RPS::Paper, Outcome::Draw) => RPS::Paper,
        (RPS::Paper, Outcome::Lose) => RPS::Rock,
        (RPS::Scissors, Outcome::Win) => RPS::Rock,
        (RPS::Scissors, Outcome::Draw) => RPS::Scissors,
        (RPS::Scissors, Outcome::Lose) => RPS::Paper,
    }
}

fn determine_move_value(rps: RPS) -> i32 {
    match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn determine_outcome_value(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line() {
        let line = "A X".to_string();
        let result = process_line(line);
        assert_eq!(result, 4);

        let line = "B Y".to_string();
        let result = process_line(line);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_aoc() {
        let result = run_aoc().unwrap();
        assert_eq!(result, (0, 0));
    }
}
