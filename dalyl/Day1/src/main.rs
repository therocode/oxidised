use text_io::read;

fn main() {
    let mut fuel: u32 = 0;

    let modules: u32 = read!();
    for module in 0..modules {
        let mass: u32 = read!();
        fuel += ((mass / 3) - 2);
    }

    println!("{}", fuel);
}
