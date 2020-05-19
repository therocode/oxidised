use std::{
    fs::File,
    io::{prelude::*, BufReader},
    error
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = File::open("modules.txt").unwrap();
    let reader = BufReader::new(file);

    let mut fuel = 0.0;
    for line in reader.lines() {
        fuel += (line?.trim().parse::<f64>()?/3.0).floor()-2.0;
    }

    println!("Fuel required for launch: {}", fuel);

    Ok(())
}
