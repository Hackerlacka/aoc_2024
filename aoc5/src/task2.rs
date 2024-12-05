use crate::utils::Printer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let file = "../input/5_1_example_input_1.txt";
        let res = run_task_with_file(file);
        assert_eq!(res, 123);
    }
}

fn run_task_with_file(file: &str) -> i64 {
    let printer = Printer::new(file);
    let incorrect_updates = printer.find_incorrect_updates();
    let corrected_updates = printer.correct_incorrect_updates(incorrect_updates);
    // This is silly...
    let tmp = corrected_updates.iter().map(|update| update).collect();
    Printer::sum_middle_pages(tmp)
}

pub fn run_task() {
    let file = "input/5_1_input.txt";
    let res = run_task_with_file(file);
    println!("Res is: {res}");
}