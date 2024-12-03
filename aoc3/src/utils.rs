use regex::Regex;

#[derive(Debug, PartialEq)]
enum InstructionType {
    Mul,
    Dont,
    Do
}

impl InstructionType {
    fn new(str: &str) -> InstructionType {
        if str.contains("mul") {
            InstructionType::Mul
        } else if str.contains("don't") {
            InstructionType::Dont
        } else if str.contains("do") {
            InstructionType::Do
        } else {
            panic!("Unknown instruction type");
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    i_type: InstructionType,
    str: String,
}

impl Instruction {
    pub fn new(str: &str) -> Instruction {
        Instruction { i_type: InstructionType::new(str), str: str.to_owned() }
    }

    pub fn evaluate_all(instructions: Vec<Instruction>) -> Vec<i64>{
        let mut res = Vec::new();
        let mut do_instruction = true;

        for instruction in instructions {
            match instruction.i_type {
                InstructionType::Mul => {
                    if !do_instruction {
                        continue;
                    }
                    res.push(instruction.evaluate());
                },
                InstructionType::Dont => {
                    do_instruction = false;
                },
                InstructionType::Do => {
                    do_instruction = true;
                }
            }
        }

        res
    }

    pub fn evaluate(&self) -> i64 {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        let caps = re.captures(&self.str).unwrap();
        let op1 = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let op2 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();

        op1 * op2
    }

    pub fn parse_valid(lines: Vec<String>, extras: bool) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        let re = if extras {
            Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))")
        } else {
            Regex::new(r"(mul\(\d{1,3},\d{1,3}\))")
        }.unwrap();
    
        for line in lines {
            for (_, [str_instruction]) in re.captures_iter(&line).map(|c| c.extract()) {
                instructions.push(Instruction::new(str_instruction));
            }
        }
    
        instructions
    }
}

