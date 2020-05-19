use std::{
    fs::File,
    io::{prelude::*, BufReader},
    error
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = File::open("modules.txt")?;
    let reader = BufReader::new(file);

    let (fuel, total_fuel) = {
        let mut sum = (0.0, 0.0);
        for line in reader.lines() {
            let mass = line?.trim().parse::<f64>()?;
            let fuel = fuel_required(mass);
            sum.0 += fuel;
            sum.1 += additional_fuel(fuel);
        }
        sum
    };

    println!("Fuel required for launching modules: {}", fuel);
    println!("Total fuel required: {}", total_fuel);

    Ok(())
}

fn fuel_required(mass: f64) -> f64
{
    return f64::max((mass/3.0).floor()-2.0, 0.0);
}

fn additional_fuel(mass: f64) -> f64
{
    let mut fuel = mass;
    let mut inc = fuel;
    while inc > 0.0 {
        inc = fuel_required(inc);
        fuel += inc;
    }
    return fuel;
}