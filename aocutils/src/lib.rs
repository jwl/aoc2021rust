use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_input_as_ints(filename: String) -> Result<Vec<i64>, Error> {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            vec.push(line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
        }
    }

    Ok(vec)
}
