use crate::utils::WordSearch;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let file = "../input/4_2_example_input_1.txt";
        let word_count = run_task_with_file(&file);
        assert_eq!(word_count, 9);
    }
}

fn run_task_with_file(file: &str) -> usize {
    let ws = WordSearch::new(file);
    ws.count_x_mas_occurence()
}

pub fn run_task() {
    let file = "input/4_1_input.txt";
    let word_count = run_task_with_file(&file);
    println!("X-MAS count is: {word_count}");
}