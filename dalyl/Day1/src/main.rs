use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("Error reading input from file");
    let reader = BufReader::new(file);

    let mut fuel: u32 = 0;
    for line in reader.lines() {
        let mass = line?.parse::<u32>().unwrap();
        fuel += ((mass / 3) - 2);
    }

    println!("{}", fuel);

    Ok(())
}
