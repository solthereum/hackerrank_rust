use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeConversion(s: &str) -> String {    
    if &s[s.len() - 2..] == "AM" {
        if &s[..2] == "12" {
            "00".to_string() + &s[2..s.len() - 2]
        } else {
            s[..s.len() - 2].to_string()
        }
    } else {
        if &s[..2] == "12" {
            s[..s.len() - 2].to_string()
        } else {
            (s[..2].parse::<i32>().unwrap() + 12).to_string() + &s[2..s.len() - 2]
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let input_string = stdin_iterator.next().unwrap().unwrap();

    let converted_time = timeConversion(&input_string);

    println!("{}", converted_time);
}