use std::io::{self, BufRead};

fn pangrams(s: &str) -> String {
    let mut flags = vec![false; 26];

    for letter in s.chars() {
        if letter.is_alphabetic() {
            flags[letter.to_ascii_lowercase() as usize - 'a' as usize] = true;
        }
    }

    for &flag in flags.iter() {
        if !flag {
            return "not pangram".to_string();
        }
    }

    "pangram".to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = pangrams(&s);

    println!("{}", result);
}
