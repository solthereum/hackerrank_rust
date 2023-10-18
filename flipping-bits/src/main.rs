use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn flipping_bits(n: i64) -> i64 {
    (u32::MAX - n as u32) as i64
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i64>().unwrap();

        let result = flipping_bits(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
