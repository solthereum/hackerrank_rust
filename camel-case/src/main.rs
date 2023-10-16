use std::io::{BufRead, self};

fn main() {
    let stdin = io::stdin();

    for line_opt in stdin.lock().lines() {
        let line = line_opt.unwrap();

        let method = &line[..1];
        let category = &line[2..3];
        let command = &line[4..];

        let mut result = String::new();
    
        if method == "S" {
            let mut is_class = category == "C";

            for letter in command.chars() {
                if letter.is_uppercase() {
                    if !is_class {
                        result.push(' ');
                    }
                    result.push(letter.to_lowercase().collect::<Vec<_>>()[0]);
                    is_class = false;
                } else if letter.is_lowercase() {
                    result.push(letter);
                }
            }
        } else {
            let mut should_capital = false;

            if category == "C" {
                should_capital = true;
            }

            for letter in command.chars() {
                if letter == ' ' {
                    should_capital = true;
                } else if should_capital == true {
                    should_capital = false;
                    result.push(letter.to_uppercase().collect::<Vec<_>>()[0]);
                } else {
                    result.push(letter);
                }
            }

            if category == "M" {
                result.push('(');
                result.push(')');
            }
        }

        println!("{}", result);
    }
}