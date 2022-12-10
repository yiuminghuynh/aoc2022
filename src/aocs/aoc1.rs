use crate::utils::read_file;
use std::error;

pub(crate) fn run_aoc() -> Result<(i32, i32), Box<dyn error::Error>> {
    let lines = read_file("resource/aoc1/input.txt")?;
    let results = process_lines(lines)?;
    let max_calorie = get_top_calorie(results.clone());
    let top_three_calories = get_top_three_calorie_total(results);

    Ok((max_calorie, top_three_calories))
}

fn process_lines(lines: Vec<String>) -> Result<Vec<i32>, Box<dyn error::Error>> {
    let mut result = vec![0];
    for line in lines {
        if line != "" {
            let last_item = result.pop().unwrap();
            let num = line.parse::<i32>()?;
            result.push(last_item + num);
        } else {
            result.push(0);
        }
    }
    Ok(result)
}

fn get_top_calorie(results: Vec<i32>) -> i32 {
    match results.into_iter().max() {
        Some(x) => x,
        None => 0,
    }
}

fn get_top_three_calorie_total(results: Vec<i32>) -> i32 {
    let mut results = results.clone();
    let mut result = 0;
    results.sort();
    results.reverse();
    for i in 0..3 {
        let cal = results[i];
        result += cal;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines() {
        let lines = vec!["1", "2", "3", "", "4", "4", "", "2"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let results = process_lines(lines);
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], 6);
        assert_eq!(results[1], 8);
        assert_eq!(results[2], 2);
    }

    #[test]
    fn test_aoc_result() {
        let result = run_aoc();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.0 > 0);
        assert!(result.1 > 0);
        print!("Top Elf: {:?}, Top Three Elves: {:?}\n", result.0, result.1)
    }
}
