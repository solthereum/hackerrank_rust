use std::io::{self, BufRead};

fn counting_valleys(steps: i32, path: &str) -> i32 {
    let mut result = 0;
    let mut height = 0;

    for step in path.to_string().chars() {
        if step == 'U' {
            height += 1;
        } else {
            if height == 0 {
                result += 1;
            }
            height -= 1;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let steps = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = counting_valleys(steps, &path);

    println!("{}", result);
}
