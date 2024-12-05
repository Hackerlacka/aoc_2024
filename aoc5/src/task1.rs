use crate::utils::Printer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_1() {
        let file = "../input/5_1_example_input_1.txt";
        let res = run_task_with_file(file);
        assert_eq!(res, 143);
    }
}

fn run_task_with_file(file: &str) -> i64 {
    let printer = Printer::new(file);
    let correct_updates = printer.find_correct_updates();
    Printer::sum_middle_pages(correct_updates)
}

pub fn run_task() {
    let file = "input/5_1_input.txt";
    let res = run_task_with_file(file);
    println!("Res is: {res}");
}