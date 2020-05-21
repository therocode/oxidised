use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn main() -> Result<(), Error> {
    let masses = read(File::open("input")?)?;

    let total_fuel: i32 = masses.iter().map(|mass| mass / 3 - 2).sum();

    println!("{}", total_fuel);

    Ok(())
}

fn read<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}
