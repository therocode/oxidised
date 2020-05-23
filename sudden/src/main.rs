// documentation here https://doc.rust-lang.org/std/index.html

// attributes with "!" means it applies to this crate.
// this file or files in this directory?

// since I'm learning, it is helpful to keep examples around.
// it's also a good idea to keep various functions I've used
// previously, so I don't have to rewrite them every single time.
#![allow(dead_code)]
// unused variables applies to arguments too.
// the compiler suggests renaming them.
// this is super annoying when introducing new arguments
// and verifying that the code builds. rust uses
// an EIGHT LINES to display such a warning.
#![allow(unused_variables)]

use std::env;
use std::fs;

// could use slices.
fn argv_strings(start: usize) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    for (i, arg) in env::args().enumerate() {
        if i < start { continue }
        v.push(arg);
    }
    return v;
}

// trying out parametric function type.
// must assert that "T" implements "Copy".
fn index_or<T: Copy>(slice: &[T], index: usize, alt: T) -> T {
    if index >= slice.len() {
        return alt
    }
    return slice[index];
}

// what is going on here? added "'lifetime" suggestions the compiler made.
fn strvec_index_or<'lifetime>(v: &'lifetime Vec<String>, i: usize, alt: &'lifetime str) -> &'lifetime str {
    // expression "if".
    if i >= v.len() { alt } else { v[i].as_str() }
}

// ... where's the arg by index function?
fn argv_index(at: usize) -> String {
    for (i, arg) in env::args().enumerate() {
        if i == at { return arg }
    }
    return "".to_string()
}

// what happens if the file read contains invalid codepoints?
// will it panic? the doc describes "String" as containing
// utf-8.
fn must_read_to_string(path: &str) -> String {
    return fs::read_to_string(path).expect("no");
}

// XXX: hardcoded input location.
// TODO: hand argv to each aoc procedure.
fn read_input(date: u32) -> String {
    let mut path = String::new();
    path.push_str("/x/devel/aoc/aoc-website/");
    path.push_str((date / 10000).to_string().as_str());
    path.push_str("/input/");
    path.push_str((date % 100).to_string().as_str());
    return must_read_to_string(path.as_str());
}

// Day 1: The Tyranny of the Rocket Equation
fn aoc20191201(args: Vec<String>) {
    fn fuel_required(mass: u32) -> u32 { mass/3 - 2 }

    // "assert_eq" is a runtime assert.
    // it will always run. very nice.
    // "debug_assert_eq" is for debug build only.
    assert_eq!(fuel_required(12), 2);
    assert_eq!(fuel_required(14), 2);
    assert_eq!(fuel_required(1969), 654);
    assert_eq!(fuel_required(100756), 33583);

    let buf = read_input(20191201);
    //let buf = must_read_to_string(args[0])

    // this is iterative. how does one map?
    let mut fuel: u64 = 0; // ... bigints?
    for s in buf.as_str().split("\n") {
        if s == "" { continue }
        let n: u64 = s.parse().expect("no");
        fuel += n;
    }

    assert_eq!(fuel, 9838570);

    println!("{}", fuel);
}

// Day 2: 1202 Program Alarm
fn aoc20191202(args: Vec<String>) {
    // turns out, the code expects values larger than 255.
    // so a u32 is needed to fit the result.
    // are there bigints in rust?
    //
    // rust seems to make integer overflow checks as evident by
    // panic "attempt to multiply with overflow" when using "Vec<u8>".
    fn exec(mem: Vec<u32>) -> Vec<u32> {
        let mut mem = mem;
        let mut pc: usize = 0;

        loop {
            let op = mem[pc];
            pc += 1;

            // no bounds checks. rust deals with it.
            match op {
                1 => {
                    // must hoist reads out of assignment or compiler
                    // will complain that immutable borrow occurs.
                    let (augend, addend, dst) = (
                        mem[mem[pc+0] as usize],
                        mem[mem[pc+1] as usize],
                        mem[pc+2] as usize,
                    );
                    pc += 3;
                    mem[dst] = augend + addend;
                },
                2 => {
                    let (multiplier, multiplicand, dst) = (
                        mem[mem[pc+0] as usize],
                        mem[mem[pc+1] as usize],
                        mem[pc+2] as usize,
                    );
                    pc += 3;
                    mem[dst] = multiplier * multiplicand;
                },

                // match expressions are ordered.
                // putting "_" at the top will catch everything.
                99 => break,
                _ => break,
            }
        }

        return mem;
    }

    fn parse(buf: &str) -> Vec<u32> {
        let mut mem: Vec<u32> = Vec::new();

        for op in buf.split(",") {
            let op = op.trim();
            if op == "" { continue }
            mem.push(op.parse().expect("no"));
        }

        // return can be omitted. I don't like it.
        // for short one-liners, for expression blocks, sure.
        return mem
    }

    assert_eq!(
        parse("1,9,10,3,2,3,11,0,\n99,30,40,50\n"),
        vec![1,9,10,3,2,3,11,0,99,30,40,50],
    );

    assert_eq!(
        exec(parse("1,0,0,0,99\n")),
        vec![2,0,0,0,99],
    );

    assert_eq!(
        exec(parse("2,3,0,3,99")),
        vec![2,3,0,6,99],
    );

    assert_eq!(
        exec(parse("2,4,4,5,99,0")),
        vec![2,4,4,5,99,9801],
    );

    assert_eq!(
        exec(parse("1,1,1,4,99,5,6,0,99")),
        vec![30,1,1,4,2,5,6,0,99],
    );

    let src = read_input(20191202);
    //let src = must_read_to_string(args[0]);

    // "12" and "2" was the generated values given by the
    // account I mirrored the aoc website with. the values
    // are different for everyone. they can be specified by
    // giving extra arguments. for example "./main 2019-12-02 0 0".
    let param1 = strvec_index_or(&args, 0, "").parse::<u32>().unwrap_or(12);
    let param2 = strvec_index_or(&args, 1, "").parse::<u32>().unwrap_or(2);

    let res = {
        let mut mem = parse(src.as_str());
        mem[1] = param1;
        mem[2] = param2;
        exec(mem)[0]
    };

    println!("{}", res);
}

// Day 3: Crossed Wires
fn aoc20191203(args: Vec<String>) {
    // not implemented
}

fn main() {
    match argv_index(1).as_str() {
    "" => println!("no"),

    "2019-12-01" => aoc20191201(argv_strings(2)),
    "2019-12-02" => aoc20191202(argv_strings(2)),
    "2019-12-03" => aoc20191203(argv_strings(2)),

    _ => (), // non-exhaustive pattern match?! "&_` not covered"
    };
}
