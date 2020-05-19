use std::{
    fs::File,
    io::{prelude::*, BufReader},
    error
};

fn fuel_required(mass: f64) -> f64
{
    return f64::max((mass/3.0).floor()-2.0, 0.0);
}

fn fuel_required_adjusted(mass: f64) -> f64
{
    let mut fuel = fuel_required(mass);
    let mut inc = fuel;
    while inc > 0.0 {
        inc = fuel_required(inc);
        fuel += inc;
    }
    return fuel;
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = File::open("modules.txt").unwrap();
    let reader = BufReader::new(file);

    let mut fuel = 0.0;
    for line in reader.lines() {
        fuel += fuel_required_adjusted(line?.trim().parse::<f64>()?);
    }

    println!("Fuel required for launching modules: {}", fuel);

    Ok(())
}
