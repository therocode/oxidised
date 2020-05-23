use std::fs;

fn main() {
    let mut opcodes: Vec<usize> = fs::read_to_string("input.txt").expect("Error reading input from file").split(",").map(|s| s.parse().expect("Parse error")).collect();

    debug_assert!(opcodes.len() >= 4);

    // Don't touch the input file
    opcodes[1] = 12;
    opcodes[2] = 2;

    for i in (0..opcodes.len()).step_by(4)  {
        let a = opcodes[i + 1];
        let b = opcodes[i + 2];
        let res = opcodes[i + 3];

        debug_assert!(a < opcodes.len() && b < opcodes.len() && res < opcodes.len(), "a = {}, b = {}, res = {}", a, b, res);

        match opcodes[i] {
            1 => {
                opcodes[res] = opcodes[a] + opcodes[b];
            }
            2 => {
                opcodes[res] = opcodes[a] * opcodes[b];
            }
            99 => {
                break;
            }
            _ => {
                // Something went wrong. RIP.
            }
        }
    }

    println!("{}", opcodes[0]);
}