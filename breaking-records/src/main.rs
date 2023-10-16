use std::io::{self, BufRead};

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let (mut max_count, mut min_count) = (0, 0);
    let (mut max_score, mut min_score) = (scores[0], scores[0]);

    for &score in scores {
        if max_score < score {
            max_score = score;
            max_count += 1;
        }

        if min_score > score {
            min_score = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
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

    let scores: Vec<i32> = stdin_iterator.next()
                                .unwrap()
                                .unwrap()
                                .trim_end()
                                .split(' ')
                                .map(|value| value.to_string().parse::<i32>().unwrap())
                                .collect();
    
    let result = breaking_records(&scores);

    println!("max: {}, min: {}", result[0], result[1]);
}