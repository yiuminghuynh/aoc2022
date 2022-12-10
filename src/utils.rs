use std::error;
use std::fs::File;
use std::io::{self, BufRead};

pub(crate) fn read_file(filename: &str) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut result = vec![];
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        result.push(line?);
    }
    Ok(result)
}
