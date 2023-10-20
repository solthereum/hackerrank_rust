use std::io::{self, BufRead};

fn counting_sort(arr: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; 100];

    for &value in arr {
        result[value as usize] += 1;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _: i32 = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let values: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = counting_sort(&values);
    let mut result_str = String::new();

    for i in 0..result.len() {
        result_str.push_str(&result[i].to_string());

        if i < result.len() - 1 {
            result_str.push(' ');
        }
    }

    println!("{}", result_str);
}
