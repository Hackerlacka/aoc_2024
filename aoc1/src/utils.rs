use regex::Regex;

fn parse_location_ids_line(line: &str) -> (i64, i64) {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    
    let caps = re.captures(line).unwrap();
    let id1 = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let id2 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
    
    (id1, id2)
}

pub fn parse_location_ids(file: &str) -> (Vec<i64>, Vec<i64>) {
    let lines = aoc_helper::read_lines(file);
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in lines {
        let (id1, id2) = parse_location_ids_line(&line);
        vec1.push(id1);
        vec2.push(id2);
    }

    return (vec1, vec2);
}