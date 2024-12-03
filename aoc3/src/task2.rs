use crate::utils::Instruction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let file = "../input/3_2_example_input.txt";
        let (evals, sum) = run_task_with_file(file);
        assert_eq!(evals, vec![2*4, 8*5]);
        assert_eq!(sum, 48);
    }
}

fn run_task_with_file(file: &str) -> (Vec<i64>, i64) {
    let lines = aoc_helper::read_lines(file);
    let instructions = Instruction::parse_valid(lines, true);

    let evals: Vec<i64> = Instruction::evaluate_all(instructions);
    let sum = evals.iter().sum();

    (evals, sum)
}

pub fn run_task() {
    let file = "input/3_1_input.txt";
    let (_, sum) = run_task_with_file(file);
    println!("Result: {sum}")
}