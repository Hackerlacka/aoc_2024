use std::{collections::HashSet, ops};

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

// TODO: Might need to implement the hash manually...
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    y: i64,
    x: i64
}

struct Guard {
    pos: Position,
    direction: Direction
}

pub struct GuardedArea {
    obstacles: HashSet<Position>,
    guard: Guard,
    area_bounds: Position
}

impl Direction {
    fn new(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Self::UP),
            '>' => Some(Self::RIGHT),
            'v' => Some(Self::DOWN),
            '<' => Some(Self::LEFT),
             _  => None
        }
    }

    fn turn_right(self: &Self) -> Direction {
        match self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP
        }
    }

    fn get_as_pos(self: &Self) -> Position {
        match self {
            Self::UP => Position { y: -1, x: 0 },
            Self::RIGHT => Position { y: 0, x: 1 },
            Self::DOWN => Position { y: 1, x: 0 },
            Self::LEFT => Position { y: 0, x: -1 },
        }
    }
}

impl Position {
    fn new(y: i64, x: i64) -> Position {
        Position { y: y, x: x }
    }
}


impl ops::Add<Position> for Position {
    type Output = Position;
    
    fn add(self, _rhs: Position) -> Position {
        Position { y: self.y + _rhs.y, x: self.x + _rhs.x }
    }
}

impl Guard {
    fn new(pos: Position, dir: Direction) -> Guard {
        Guard { pos: pos, direction: dir }
    }

    fn do_move(self: &mut Self, area_bounds: &Position, obstacles: &HashSet<Position>) -> bool {
        let dpos = self.direction.get_as_pos();
        let new_pos = self.pos + dpos;
        
        if GuardedArea::is_out_of_bounds(area_bounds, &new_pos) {
            return false;
        } else if GuardedArea::is_obstacle(obstacles, &new_pos) {
            self.direction = self.direction.turn_right();
        } else {
            self.pos = new_pos;
        }

        return true;
    }
}

impl GuardedArea {
    pub fn new(file: &str) -> GuardedArea {
        let lines = aoc_helper::read_lines(file);
        let area_bounds = Position::new(lines.len() as i64, lines.iter().last().unwrap().len() as i64); 
        let mut guard = None;
        let mut obstacles = HashSet::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let dir = Direction::new(c);
                if c == '#' {
                    obstacles.insert(Position { y: y as i64, x: x as i64 });
                } else if dir.is_some() {
                    guard = Some(Guard::new(Position::new(y as i64, x as i64), dir.unwrap()));
                }
            }
        }

        GuardedArea { obstacles: obstacles, guard: guard.unwrap(), area_bounds: area_bounds }
    }

    fn is_out_of_bounds(area_bounds: &Position, pos: &Position) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x >= area_bounds.x || pos.y >= area_bounds.y
    }

    fn is_obstacle(obstacles: &HashSet<Position>, pos: &Position) -> bool {
        obstacles.contains(pos)
    }

    pub fn find_distinct_guard_pos(self: &mut Self) -> u64 {
        let mut unique_positions = HashSet::new();
        loop {
            // Guard moves
            if self.guard.do_move(&self.area_bounds, &self.obstacles) {
                unique_positions.insert(self.guard.pos.clone());
            } else {
                // Next position would have been out of bounds
                return unique_positions.len() as u64;
            }
        }
    }
}