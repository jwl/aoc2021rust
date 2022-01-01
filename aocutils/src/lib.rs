use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_input_as_ints(filename: String) -> Result<Vec<i64>, Error> {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            vec.push(
                line?
                    .trim()
                    .parse()
                    .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
            );
        }
    }

    Ok(vec)
}

pub fn load_input_as_strings(filename: String) -> Vec<String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap()
}

// Read in file with an int in each line and and output a Vector
pub fn read_input_as_ints(filename: String) -> Vec<i64> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}

pub fn load_oneline_as_ints(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

pub fn read_input_as_binaryints(filename: String) -> Vec<i64> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| i64::from_str_radix(line.unwrap().as_str(), 2).unwrap())
        .collect()
}

pub fn read_file_to_string(filename: String) -> String {
    let file = fs::read_to_string(&filename).expect("Please include input file!");
    file.replace("\r\n", "\n")
}
