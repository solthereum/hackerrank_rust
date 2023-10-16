use std::io::{self, BufRead};

fn mini_max_sum(arr: &[i32]) {
    let mut sum: i64 = 0;
    let mut max: i32 = 0;
    let mut min: i32 = 1_000_000_000;

    for &value in arr {
        if value > max {
            max = value;
        }

        if value < min {
            min = value;
        }

        sum += value as i64;
    }

    println!("{} {}", sum - max as i64, sum - min as i64);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    mini_max_sum(&arr);
}
