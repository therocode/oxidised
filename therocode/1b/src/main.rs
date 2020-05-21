use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn fuel_needed(mass: &i32) -> i32 {
    mass / 3 - 2
}

struct NeededExtraFuelIter {
    last_added: i32,
}

impl NeededExtraFuelIter {
    fn new(start: i32) -> NeededExtraFuelIter {
        NeededExtraFuelIter { last_added: start }
    }
}

impl Iterator for NeededExtraFuelIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let to_add = match fuel_needed(&self.last_added) {
            x if x > 0 => x,
            _ => 0,
        };
        self.last_added = to_add;

        if self.last_added == 0 {
            None
        } else {
            Some(self.last_added)
        }
    }
}

fn main() -> Result<(), Error> {
    let masses = read(File::open("input")?)?;

    let mut total_fuel: i32 = masses.iter().map(fuel_needed).sum();

    let fuel_iter = NeededExtraFuelIter::new(total_fuel);

    for extra_fuel in fuel_iter {
        total_fuel += extra_fuel;
        println!("Added {} to fuel, total is now {}", extra_fuel, total_fuel)
    }

    println!("{}", total_fuel);

    Ok(())
}

fn read<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}
