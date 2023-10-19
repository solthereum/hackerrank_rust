use std::io::{self, BufRead};

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let mut result = 0;
    let sum = arr.len() - 1;

    for (i, line) in arr.into_iter().enumerate() {
        for (j, &value) in line.into_iter().enumerate() {
            if i == j {
                result += value;
            }
            
            if i + j == sum {
                result -= value;
            }
        }
    }

    result.abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonal_difference(&arr);

    println!("{}", result);
}
