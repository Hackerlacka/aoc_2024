use crate::utils::WordSearch;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let file = "../input/4_1_example_input_1.txt";
        let word_count = run_task_with_file(&file);
        assert_eq!(word_count, 4);
    }

    #[test]
    fn test_example_input_2() {
        let file = "../input/4_1_example_input_2.txt";
        let word_count = run_task_with_file(&file);
        assert_eq!(word_count, 18);
    }
}

fn run_task_with_file(file: &str) -> usize {
    let ws = WordSearch::new(file);
    ws.count_xmas_occurence()
}

pub fn run_task() {
    let file = "input/4_1_input.txt";
    let word_count = run_task_with_file(&file);
    println!("Word count is: {word_count}");
}