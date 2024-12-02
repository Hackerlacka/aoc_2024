use crate::utils::Report;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let file = "../input/2_1_example_input.txt";
        let safe_reports = run_task_with_file(file);
        assert_eq!(safe_reports, 2);
    }
}

fn run_task_with_file(file: &str) -> usize {
    let mut reports = Report::parse_reports(file);
    Report::filter_safe_reports(&mut reports, false);
    reports.len()
}

pub fn run_task() {
    let file = "input/2_1_input.txt";
    let safe_reports = run_task_with_file(file);
    println!("Safe reports: {safe_reports}");
}