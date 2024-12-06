use crate::utils::GuardedArea;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        //env_logger::init();

        let file = "../input/6_1_example_input_1.txt";
        let loops = run_task_with_file(file);
        assert_eq!(loops, 6);
    }
}

fn run_task_with_file(file: &str) -> u64 {
    let mut guarded_area = GuardedArea::new(file);
    GuardedArea::find_all_loops(&mut guarded_area)
}

pub fn run_task() {
    let file = "input/6_1_input.txt";
    let loops = run_task_with_file(file);
    println!("Loops: {loops}");
}