use std::io;
use std::io::Write;

fn main() {
    let mut mass_str = String::new();
    
    print!("Enter mass of module: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut mass_str).unwrap(); 
    let mass: f64 = mass_str.trim().parse().unwrap();

    let fuel = (mass/3.0).floor() - 2.0;
    println!("Fuel required for launch: {}", fuel as i64);
}
