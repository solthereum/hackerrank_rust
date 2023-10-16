use std::io::{self, BufRead};

fn plus_minus(arr: &[i32]) {
    let length = arr.len();
    let mut zero_count = 0;
    let mut plus_count = 0;
    let mut minus_count = 0;

    for digit in arr {
        if *digit > 0 {
            plus_count += 1;
        } else if *digit < 0 {
            minus_count += 1;
        } else {
            zero_count += 1;
        }
    }

    // println!("zero_count: {}, plus_count: {}, minus_count: {}", zero_count, plus_count, minus_count);

    println!("{:.6}", plus_count as f32 / length as f32);
    println!("{:.6}", minus_count as f32 / length as f32);
    println!("{:.6}", zero_count as f32 / length as f32);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _ = stdin_iterator.next()
                    .unwrap()
                    .unwrap()
                    .trim()
                    .parse::<i32>()
                    .unwrap();
    let arr: Vec<i32> = stdin_iterator.next()
                            .unwrap()
                            .unwrap()
                            .trim_end()
                            .split(' ')
                            .map(|s| s.to_string().parse::<i32>().unwrap())
                            .collect();

                            plus_minus(&arr);
}
