use std::io::{self, prelude::*, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("Error reading input from file");
    let reader = BufReader::new(file);

    let mut total_fuel = 0;
    for line in reader.lines() {
        let mass = line?.parse::<i32>().unwrap();
        total_fuel += additional_fuel(mass);
    }

    println!("{}", total_fuel);

    Ok(())
}

fn additional_fuel(mut mass: i32) -> i32 {
    let mut fuel = 0;
    while mass > 0 {
        mass = (mass / 3) - 2;
        if mass > 0 {
            fuel += mass;
        }
    }
    return fuel;
}