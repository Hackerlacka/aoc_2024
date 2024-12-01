use std::collections::HashMap;

use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let file = "../input/1_1_example_input.txt";
        let similarity_score = run_task_with_file(file);
        assert_eq!(similarity_score, 31);
    }
}

fn find_count(vec: &Vec<i64>, number: &i64) -> i64 {
    vec.iter().filter(|n | {
        *n == number
    }).count() as i64
}

fn calc_similarity_score(vec1: Vec<i64>, vec2: Vec<i64>) -> i64 {
    let mut cache: HashMap<i64, i64> = HashMap::new();

    vec1.iter().map(|number| {
        match cache.get(number) {
            Some(count) => number * count.to_owned(),
            None => {
                let count = find_count(&vec2, number);
                cache.insert(*number, count);
                number * count.to_owned()
            }
        }
    }).sum()
}

fn run_task_with_file(file: &str) -> i64 {
    // Read lines with numbers -> Two lists
    let (vec1, vec2) = utils::parse_location_ids(file);

    // Calculate similarity score
    calc_similarity_score(vec1, vec2)
}

pub fn run_task() {
    let file = "input/1_1_input.txt";
    let diffs = run_task_with_file(file);
    println!("Result: {diffs}");
}