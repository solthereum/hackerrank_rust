use std::io::{self, BufRead};

fn mars_exploration(s: &str) -> i32 {
    let mut result = 0;

    for (index, letter) in s.chars().enumerate() {
        if index % 3 != 1 && letter != 'S'
            || index % 3 == 1 && letter != 'O' 
        {
            result += 1;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = mars_exploration(&s);

    println!("{}", result);
}
