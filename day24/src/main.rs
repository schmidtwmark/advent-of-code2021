use itertools::Itertools;
use std::error;
use std::fmt;
use std::{collections::HashMap, env, fs};

#[derive(Copy, Clone, Debug)]
enum Target {
    Variable(char),
    Number(i64),
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Input(char), // Read input from program input
    Add(char, Target),
    Multiply(char, Target),
    Divide(char, Target),
    Mod(char, Target),
    Equal(char, Target),
}

type Input<'a> = Box<dyn Iterator<Item = &'a i64> + 'a>;
struct Executor<'a> {
    program: &'a [Instruction],
    registry: HashMap<char, i64>,
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct InvalidVariable {
    variable: char,
}

impl fmt::Display for InvalidVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find variable {}", self.variable)
    }
}

impl error::Error for InvalidVariable {}

#[derive(Debug, Clone)]
struct DivideByZero {}
impl fmt::Display for DivideByZero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot divide by zero!")
    }
}
impl error::Error for DivideByZero {}

#[derive(Debug, Clone)]
struct NoInput {}
impl fmt::Display for NoInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No more input")
    }
}
impl error::Error for NoInput {}

#[derive(Debug, Clone)]
struct Overflow {}
impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No more input")
    }
}
impl error::Error for Overflow {}

impl Executor<'_> {
    fn new<'a>(instructions: &'a [Instruction]) -> Executor<'a> {
        Executor {
            program: instructions,
            registry: HashMap::new(),
        }
    }

    fn process_instruction<'a, I>(&mut self, instruction: &Instruction, mut input: I) -> Result<()>
    where
        I: Iterator<Item = &'a i64>,
    {
        println!("Processing instruction {:?}", instruction);
        match instruction {
            Instruction::Input(a) => {
                *self.registry.entry(*a).or_default() =
                    *input.next().ok_or_else(|| Box::new(NoInput {}))?;
            }
            Instruction::Add(a, b) => {
                let b = self.get_target(b);
                let a = self.at(a);
                *a = a.checked_add(b).ok_or_else(|| Box::new(Overflow {}))?;
            }
            Instruction::Multiply(a, b) => {
                let b = self.get_target(b);
                let a = self.at(a);
                *a = a.checked_mul(b).ok_or_else(|| Box::new(Overflow {}))?;
            }
            Instruction::Divide(a, b) => {
                let b = self.get_target(b);
                let a = self.at(a);
                *a = a.checked_div(b).ok_or_else(|| Box::new(DivideByZero {}))?;
            }
            Instruction::Mod(a, b) => {
                let b = self.get_target(b);
                let a = self.at(a);
                *a = a.checked_rem(b).ok_or_else(|| Box::new(DivideByZero {}))?;
            }
            Instruction::Equal(a, b) => {
                let b = self.get_target(b);
                let a = self.at(a);
                if *a == b {
                    *a = 1
                } else {
                    *a = 0
                };
            }
        };
        Ok(())
    }

    fn process<'a, I>(&mut self, mut input: I) -> Result<()>
    where
        I: Iterator<Item = &'a i64>,
    {
        self.registry.clear();
        for instruction in self.program {
            if let Err(e) = self.process_instruction(instruction, &mut input) {
                println!("Failed with:\nInstruction: {:?}\nRegistry:", instruction);
                self.print_registry();
                return Err(e);
            }
        }
        Ok(())
    }

    fn get_target(&mut self, b: &Target) -> i64 {
        match b {
            Target::Number(n) => *n,
            Target::Variable(c) => *self.at(c),
        }
    }

    fn at(&mut self, variable: &char) -> &mut i64 {
        self.registry.entry(*variable).or_default()
    }

    fn print_registry(&self) {
        self.registry
            .iter()
            .sorted_by_key(|(c, v)| **c)
            .for_each(|(c, v)| println!("({}: {})", c, v));
    }
}

fn main() -> Result<()> {
    let (filename, _sample_param) = if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let input_lines = contents.split('\n');

    let instructions = input_lines
        .filter_map(|line| {
            let mut words = line.split(' ');
            let instruction = words.next()?;
            let ident = words.next()?.chars().next()?;
            let params = words
                .filter_map(|s| {
                    if let Ok(n) = s.parse() {
                        Some(Target::Number(n))
                    } else {
                        Some(Target::Variable(s.chars().next()?))
                    }
                })
                .collect_vec();
            Some(match instruction {
                "inp" => Instruction::Input(ident),
                "add" => Instruction::Add(ident, params[0]),
                "mul" => Instruction::Multiply(ident, params[0]),
                "div" => Instruction::Divide(ident, params[0]),
                "mod" => Instruction::Mod(ident, params[0]),
                "eql" => Instruction::Equal(ident, params[0]),
                _ => unreachable!(),
            })
        })
        .collect_vec();

    // println!("Instructions: {:?}", instructions);
    let mut executor = Executor::new(&instructions);
    while {
        let input = (0..14).collect_vec();
        executor.process(input.iter())?;
        false
    } {}

    executor.print_registry();
    Ok(())
}
