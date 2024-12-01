use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let file = "../input/1_1_example_input.txt";
        let diffs = run_task_with_file(file);
        assert_eq!(diffs, 11);
    }
}

fn calc_location_ids_diffs(mut vec1: Vec<i64>, mut vec2: Vec<i64>) -> i64 {
    // Sort lists
    vec1.sort();
    vec2.sort();

    let mut diffs = 0;
    for (id1, id2) in vec1.iter().zip(vec2.iter()) {
        diffs += (id1 - id2).abs();
    }

    diffs
}

fn run_task_with_file(file: &str) -> i64 {
    // Read lines with numbers -> Two lists
    let (vec1, vec2) = utils::parse_location_ids(file);

    // Calculate differences and sum them
    calc_location_ids_diffs(vec1, vec2)
}

pub fn run_task() {
    let file = "input/1_1_input.txt";
    let diffs = run_task_with_file(file);
    println!("Result: {diffs}");
}