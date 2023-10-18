use std::io::{self, BufRead};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(grades.len());

    for &grade in grades {
        if grade > 37 && grade % 5 > 2 {
            result.push(((grade / 5) as i32 + 1) * 5);
        } else {
            result.push(grade);
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let grades: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = grading_students(&grades);

    for i in 0..result.len() {
        println!("{}", result[i]);
    }
}
