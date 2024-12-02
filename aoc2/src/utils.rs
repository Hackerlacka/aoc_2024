#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    levels: Vec<i64>,
}

impl Report {
    pub fn new(line: &str) -> Report {
        let mut levels = Vec::new();

        let splits = line.split_whitespace();
        for level in splits {
            levels.push(level.parse::<i64>().unwrap());
        }

        Self { levels: levels }
    }

    fn are_levels_safe(levels: &Vec<i64>) -> (bool, usize) {
        let mut iter = levels.iter().enumerate();
        let mut current = iter.next().unwrap().1;
        let mut increasing: Option<bool> = None;

        for (index, next ) in iter {
            let diff = current - next;
            if increasing.is_none() {
                increasing = Some(diff.is_negative());
            }
            
            if diff == 0 || diff.abs() > 3 || diff.is_negative() != increasing.unwrap() {
                return (false, index)
            }
            current = next;
        }
        
        return (true, 0);
    }

    fn is_safe_remove(self: &Self, index: usize) -> bool {
        let mut levels = self.levels.clone();
        levels.remove(index);

        let (safe, _) = Self::are_levels_safe(&levels);
        return safe;
    }

    pub fn is_safe(self: &Self, tolerance: bool) -> bool {
        let (safe, index) = Self::are_levels_safe(&self.levels);
        if safe {
            return true;
        } else if !tolerance {
            return false;
        }

        // The '2' covers the special case of e.g. "[12 11 13 15 18 20]"
        let limit = if index == 2 { 2 } else { 1 };

        // See if error is tolerated by removing some numbers
        for i in 0..=limit {
            if i > index {
                break;
            } else if Self::is_safe_remove(self, index - i) {
                return true;
            }
        }

        return false;
    }

    pub fn parse_reports(file: &str) -> Vec<Report> {
        let lines = aoc_helper::read_lines(file);
    
        lines.iter().map(|line| {
            Report::new(line)
        }).collect()
    }

    pub fn filter_safe_reports(reports: &mut Vec<Report>, tolerance: bool) {
        reports.retain(|report| report.is_safe(tolerance));
    }

}