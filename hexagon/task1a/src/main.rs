use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("modules.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut fuel: f64 = 0.0;

    for line in reader.lines() {
        let mass: f64 = line.unwrap().trim().parse().unwrap();
        fuel += (mass/3.0).floor() - 2.0;
    }

    println!("Fuel required for launch: {}", fuel as i64);
}
