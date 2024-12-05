use log::debug;

#[derive(Debug)]
pub struct OrderingRule {
    x: i64,
    y: i64
}

#[derive(Debug, Clone)]
pub struct Update {
    pages: Vec<i64>
}

#[derive(Debug)]
pub struct Printer {
    ordering_rules: Vec<OrderingRule>,
    updates: Vec<Update>
}

impl OrderingRule {
    fn new(line: &str) -> OrderingRule {
        let split: Vec<&str> = line.split('|').collect();
        let x = split.get(0).unwrap().parse::<i64>().unwrap();
        let y = split.get(1).unwrap().parse::<i64>().unwrap();
        OrderingRule { x: x, y: y }
    }
}

impl Update {
    fn new(line: &str) -> Update {
        let pages = line.split(',').map(|str| str.parse::<i64>().unwrap()).collect();
        Update { pages: pages }
    }

    fn is_correct(self: &Self, ordering_rules: &Vec<OrderingRule>) -> bool {
        debug!("Testing: {self:?}");
        for page in self.pages.iter() {
            for ordering_rule in ordering_rules.iter() {
                if *page == ordering_rule.x {
                    let x_pos = self.pages.iter().position(|tmp_page| tmp_page == page).unwrap();
                    let y_pos = self.pages.iter().position(|tmp_page| *tmp_page == ordering_rule.y);
                    if y_pos.is_none() {
                        continue;
                    }
                    debug!("page: {page}, ordering_rule: {ordering_rule:?}, x_pos: {x_pos} y_pos: {}",y_pos.unwrap());
                    if y_pos.unwrap() < x_pos {
                        debug!("Failing: {self:?}");
                        return false;
                    }
                }
            }
        }

        debug!("Successful: {self:?}");
        return true;
    }

    fn do_move(self: &mut Self, ordering_rule: &OrderingRule) -> bool {
        let x_pos = self.pages.iter().position(|tmp_page| *tmp_page == ordering_rule.x).unwrap();
        let y_pos = self.pages.iter().position(|tmp_page| *tmp_page == ordering_rule.y).unwrap();
        if y_pos < x_pos {
            // Perform move
            let page = self.pages.remove(x_pos);
            self.pages.insert(y_pos, page);
            return true;
        }

        return false;
    }

    fn do_correct(self: &mut Self, ordering_rules: &Vec<OrderingRule>) {
        'outer: loop {
            for i in 0..self.pages.len() {
                let page = *self.pages.get(i).unwrap();
                for j in 0..ordering_rules.len() {
                    // Apply each ordering rule one by one if they apply to the printer update pages
                    let ordering_rule = ordering_rules.get(j).unwrap();
                    if ordering_rule.x != page || !self.pages.contains(&ordering_rule.y) {
                        continue; 
                    }
                    
                    if self.do_move(ordering_rule) {
                        // If a move was performed restart everything ("efficient" :D)
                        continue 'outer;
                    }
                }
            }
            break;  
        }
    }
}

impl Printer {
    pub fn new(file: &str) -> Printer {
        let lines = aoc_helper::read_lines(file);
        let mut ordering_rules = Vec::new();
        let mut updates = Vec::new();

        let mut found_separator = false;
        for line in lines {
            if line.is_empty() {
                found_separator = true;
            } else if !found_separator {
                ordering_rules.push(OrderingRule::new(&line));
            } else {
                updates.push(Update::new(&line));
            }
        }

        Printer { ordering_rules: ordering_rules, updates: updates }
    }

    pub fn find_correct_updates(self: &Self) -> Vec<&Update> {
        self.updates.iter().filter(|update| update.is_correct(&self.ordering_rules)).collect()
    }

    pub fn find_incorrect_updates(self: &Self) -> Vec<Update> {
        let updates: Vec<&Update> = self.updates.iter().filter(|update| !update.is_correct(&self.ordering_rules)).collect();
        updates.iter().map(|update| (*update).clone()).collect()
    }

    pub fn correct_incorrect_updates(self: &Self, mut updates: Vec<Update>) -> Vec<Update> {
        updates.iter_mut().for_each(|update| update.do_correct(&self.ordering_rules));
        updates
    }

    pub fn sum_middle_pages(updates: Vec<&Update>) -> i64 {
        updates.iter().map(|update| update.pages.get(update.pages.len()/2).unwrap()).sum()
    }
}