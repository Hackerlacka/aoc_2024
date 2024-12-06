use crate::utils::GuardedArea;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        //env_logger::init();

        let file = "../input/6_1_example_input_1.txt";
        let distinct_pos = run_task_with_file(file);
        assert_eq!(distinct_pos, 41);
    }
}

fn run_task_with_file(file: &str) -> u64 {
    let mut guarded_area = GuardedArea::new(file);
    guarded_area.find_distinct_guard_pos()
}

pub fn run_task() {
    let file = "input/6_1_input.txt";
    let distinct_pos = run_task_with_file(file);
    println!("Distinct positions: {distinct_pos}");
}