use std::io::{self, BufRead};

fn matching_strings(strings: &[String], queries: &[String]) -> Vec<i32> {
    let mut counts: Vec<i32> = vec![0; queries.len()];

    for (index, query) in queries.into_iter().enumerate() {
        for string in strings {
            if query == string {
                counts[index] += 1;
            }
        }
    }

    counts
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let strings_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut strings: Vec<String> = Vec::with_capacity(strings_count as usize);

    for _ in 0..strings_count {
        let strings_item = stdin_iterator.next().unwrap().unwrap();
        strings.push(strings_item);
    }

    let queries_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut queries: Vec<String> = Vec::with_capacity(queries_count as usize);

    for _ in 0..queries_count {
        let queries_item = stdin_iterator.next().unwrap().unwrap();
        queries.push(queries_item);
    }

    let res = matching_strings(&strings, &queries);

    for i in 0..res.len() {
        println!("{}", res[i]);
    }
}
