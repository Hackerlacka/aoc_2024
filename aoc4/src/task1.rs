
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

struct WordSearch {
    word_map: Vec<Vec<char>>
}

impl WordSearch {
    fn new(file: &str) -> WordSearch {
        let lines = aoc_helper::read_lines(file);
        let word_map = lines.iter().map(|line| line.chars().into_iter().collect()).collect();
        WordSearch { word_map: word_map }
    }

    fn check_position(self: &Self, y_pos: usize, x_pos: usize) -> usize {
        let len = 4;

        let mut strs = Vec::new();
        let mut tmp = String::new();

        /* Check all directions (Looks like shit! Hashmap would be cleaner) */
        for x in 1..len {
            tmp.push(self.word_map[y_pos].get(x_pos + x).unwrap_or(&'.').to_owned());
        }
        strs.push(tmp);
        tmp = String::new();

        for x in 1..len {
            tmp.push(self.word_map[y_pos].get(x_pos - x).unwrap_or(&'.').to_owned());
        }
        strs.push(tmp);
        tmp = String::new();

        for y in 1..len {
            tmp.push(self.word_map.get(y_pos + y).unwrap_or(&vec!['.']).get(x_pos).unwrap_or(&'.').to_owned());
        }
        strs.push(tmp);
        tmp = String::new();

        for y in 1..len {
            tmp.push(self.word_map.get(y_pos - y).unwrap_or(&vec!['.']).get(x_pos).unwrap_or(&'.').to_owned());
        }
        strs.push(tmp);
        tmp = String::new();

        /* Diagonal */
        for xy in 1..len {
            tmp.push(self.word_map.get(y_pos + xy).unwrap_or(&vec!['.']).get(x_pos + xy).unwrap_or(&'.').to_owned());
        }
        strs.push(tmp);
        tmp = String::new();

        for xy in 1..len {
            tmp.push(self.word_map.get(y_pos - xy).unwrap_or(&vec!['.']).get(x_pos - xy).unwrap_or(&'.').to_owned());
        }
        strs.push(tmp);
        tmp = String::new();

        for xy in 1..len {
            tmp.push(self.word_map.get(y_pos - xy).unwrap_or(&vec!['.']).get(x_pos + xy).unwrap_or(&'.').to_owned());
        }
        strs.push(tmp);
        tmp = String::new();

        for xy in 1..len {
            tmp.push(self.word_map.get(y_pos + xy).unwrap_or(&vec!['.']).get(x_pos - xy).unwrap_or(&'.').to_owned());
        }
        strs.push(tmp);

        /* Check how many were actually X(MAS) */
        strs.retain(|line| line == "MAS");
        strs.len()
    }

    fn count_xmas_occurence(self: &Self) -> usize {
        let mut count = 0;
        for (y, line) in self.word_map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c != 'X' {
                    continue;
                }
                count += self.check_position(y, x);
            }
        }

        count
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