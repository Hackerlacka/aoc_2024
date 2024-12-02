use crate::utils::Report;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let file = "../input/2_1_example_input.txt";
        let safe_reports = run_task_with_file(file);
        assert_eq!(safe_reports, 4);
    }

    #[test]
    fn test_custom_input_1() {
        // [12 11 13 15 18 20]
        let file = "../input/2_2_custom_input_1.txt";
        let safe_reports = run_task_with_file(file);
        assert_eq!(safe_reports, 1);
    }
}


fn run_task_with_file(file: &str) -> usize {
    let mut reports = Report::parse_reports(file);
    Report::filter_safe_reports(&mut reports, true);
    reports.len()
}

pub fn run_task() {
    let file = "input/2_1_input.txt";
    let safe_reports = run_task_with_file(file);
    println!("Safe reports: {safe_reports}");
}