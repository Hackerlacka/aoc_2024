use std::collections::HashMap;

pub struct WordSearch {
    word_map: HashMap<String, char>,
    size: (i64, i64)
}

impl WordSearch {
    pub fn new(file: &str) -> WordSearch {
        let lines = aoc_helper::read_lines(file);
        let size = (lines.iter().len() as i64, lines.iter().next().unwrap().len() as i64);
        let mut word_map = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                word_map.insert(format!("{y},{x}"), c);
            }
        }

        WordSearch { word_map: word_map, size }
    }

    fn check_position(self: &Self, y_pos: i64, x_pos: i64, word: &str, diag_only: bool) -> Vec<(usize, usize)> {
        let len = word.len() as i64;
        let hlen = len/2;

        let mut strs: Vec<((i64, i64), String)> = Vec::new();
        let mut tmp = String::new();

        /* Check all directions */
        if !diag_only {
            for x in 1..len {
                tmp.push(self.word_map.get(&format!("{y_pos},{}", x_pos + x)).unwrap_or(&'.').to_owned());
            }
            strs.push(((y_pos, x_pos + hlen), tmp));
            tmp = String::new();
    
            for x in 1..len {
                tmp.push(self.word_map.get(&format!("{y_pos},{}", x_pos - x)).unwrap_or(&'.').to_owned());
            }
            strs.push(((y_pos, x_pos - hlen), tmp));
            tmp = String::new();
    
            for y in 1..len {
                tmp.push(self.word_map.get(&format!("{},{x_pos}", y_pos + y)).unwrap_or(&'.').to_owned());
            }
            strs.push(((y_pos + hlen, x_pos), tmp));
            tmp = String::new();
    
            for y in 1..len {
                tmp.push(self.word_map.get(&format!("{},{x_pos}", y_pos - y)).unwrap_or(&'.').to_owned());
            }
            strs.push(((y_pos - hlen, x_pos), tmp));
            tmp = String::new();
        }

        /* Diagonal */
        for xy in 1..len {
            tmp.push(self.word_map.get(&format!("{},{}", y_pos + xy, x_pos + xy)).unwrap_or(&'.').to_owned());
        }
        strs.push(((y_pos + hlen, x_pos + hlen), tmp));
        tmp = String::new();

        for xy in 1..len {
            tmp.push(self.word_map.get(&format!("{},{}", y_pos - xy, x_pos - xy)).unwrap_or(&'.').to_owned());
        }
        strs.push(((y_pos - hlen, x_pos - hlen), tmp));
        tmp = String::new();

        for xy in 1..len {
            tmp.push(self.word_map.get(&format!("{},{}", y_pos - xy, x_pos + xy)).unwrap_or(&'.').to_owned());
        }
        strs.push(((y_pos - hlen, x_pos + hlen), tmp));
        tmp = String::new();

        for xy in 1..len {
            tmp.push(self.word_map.get(&format!("{},{}", y_pos + xy, x_pos - xy)).unwrap_or(&'.').to_owned());
        }
        strs.push(((y_pos + hlen, x_pos - hlen), tmp));

        /* Check how many were actually W(ORD) */
        strs.retain(|(_, tmp_word)| tmp_word.as_str() == &word[1..]);
        strs.iter().map(|entry| (entry.0.0 as usize, entry.0.1 as usize)).collect()
    }

    pub fn count_xmas_occurence(self: &Self) -> usize {
        let mut count = 0;
        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                if self.word_map.get(&format!("{y},{x}")).unwrap_or(&'.') != &'X' {
                    continue;
                }
                count += self.check_position(y, x, "XMAS", false).len();
            }
        }

        count
    }

    pub fn count_x_mas_occurence(self: &Self) -> usize {
        let mut hash_pos = HashMap::new();
        let word = "MAS";
        let first_letter = word.chars().next().unwrap();

        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                if self.word_map.get(&format!("{y},{x}")).unwrap_or(&'.') != &first_letter {
                    continue;
                }
                let positions = self.check_position(y, x, word, true);
                for pos in positions {
                    let key = format!("{},{}", pos.0, pos.1);
                    let prev_val = hash_pos.get(&key).unwrap_or(&0);
                    hash_pos.insert(key, *prev_val + 1);
                }
            }
        }

        hash_pos.iter().filter(|(_, v)| {
            **v >= 2
        }).count()

        
    }
}