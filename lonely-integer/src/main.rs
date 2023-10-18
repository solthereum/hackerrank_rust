use std::io::{self, BufRead};

fn lonely_integer(a: &[i32]) -> i32 {
    let mut flags: Vec<bool> = vec![false; a.len()];

    let mut result = 0;

    for index in 0..a.len() {
        if !flags[index] {
            for pair_index in (index + 1)..a.len() {
                if a[index] == a[pair_index] {
                    flags[index] = true;
                    flags[pair_index] = true;
                    
                    break;
                }
            }
        }
    }

    for index in 0..a.len() {
        if !flags[index] {
            result = a[index];

            break;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _ = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = lonely_integer(&a);

    println!("{}", result);
}
