use itertools::Itertools;
use std::fmt;
use std::{collections::HashMap, env, fs};
use std::{error, ops::Mul};

#[derive(Copy, Clone, Debug)]
enum Target {
    Variable(char),
    Number(i64),
}

impl Target {
    fn get_str(&self) -> String {
        match self {
            Self::Variable(c) => format!("{}", c),
            Self::Number(n) => n.to_string(),
        }
    }
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

impl Instruction {
    fn process(&self, registry: &mut Registry, input: i64) {
        match self {
            Instruction::Input(a) => *registry.at(a).unwrap() = input,
            Instruction::Add(a, b) => {
                let b = registry.get_target(b);
                let a = registry.at(a).unwrap();
                *a = *a + b;
            }
            Instruction::Multiply(a, b) => {
                let b = registry.get_target(b);
                let a = registry.at(a).unwrap();
                *a = *a * b;
            }
            Instruction::Divide(a, b) => {
                let b = registry.get_target(b);
                let a = registry.at(a).unwrap();
                *a = *a / b;

            }
            Instruction::Mod(a, b) => {
                let b = registry.get_target(b);
                let a = registry.at(a).unwrap();
                *a = *a % b;
            }
            Instruction::Equal(a, b) => {
                let b = registry.get_target(b);
                let a = registry.at(a).unwrap();
                if *a == b {
                    *a = 1
                } else {
                    *a = 0
                };
            }
        };
    }

}

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
    fn new(instructions: &[Instruction]) -> Executor {
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
        self.print_registry();
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
    fn to_rust(&self) -> Vec<String> {
        use Instruction::*;
        self.program
            .iter()
            .map(|instruction| match instruction {
                Input(c) => format!("{} = *input.next().unwrap();", c),
                Add(c, t) => format!("{} += {};", c, t.get_str()),
                Multiply(c, t) => format!("{} *= {};", c, t.get_str()),
                Divide(c, t) => format!("{} /= {};", c, t.get_str()),
                Mod(c, t) => format!("{} %= {};", c, t.get_str()),
                Equal(c, t) => {
                    format!("{} = if {} == {} {{ 1 }} else {{ 0 }};", c, c, t.get_str())
                }
            })
            .collect_vec()
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
            .sorted_by_key(|(c, _v)| **c)
            .for_each(|(c, v)| println!("({}: {})", c, v));
    }
}

fn f(
    w: &mut i64,
    x: &mut i64,
    y: &mut i64,
    z: &mut i64,
    i: i64,
    x_offset: i64,
    y_offset: i64,
    z_offset: i64,
) {
    *w = i; // read from input to 0

    // Set x to z mod 26
    *x = *z % 26;
    *z /= z_offset; // Divide z by 1 or 26
    *x += x_offset; // Add an offset to x
    println!("a: ({}, {}, {}, {})", w, x, y, z);

    // x = 1 iff x != w else 0
    if *x == *w {
        *x = 0;
        *y = 0;
    } else {
        *x = 1;
        *y = 25 * *x + 1;
        *z *= *y;

        *y = (*w + y_offset) * *x;
        *z += *y;
    }
    // *x = if x == w { 1 } else { 0 };
    // *x = if *x == 0 { 1 } else { 0 };
    // println!("b: ({}, {}, {}, {})", w, x, y, z);

    // // if x == w { y = (25 * x + 1) } else { y = 1}
    // *y *= 0;
    // *y += 25;
    // *y *= *x;
    // *y += 1;
    // println!("c: ({}, {}, {}, {})", w, x, y, z);

    // // multiply z times y
    // // How to get y to 0?
    // *z *= *y;
    // println!("d: ({}, {}, {}, {})", w, x, y, z);

    // // store in y (input + y_offset) * x
    // *y *= 0;
    // *y += *w;
    // *y += y_offset;
    // *y *= *x;
    // println!("e: ({}, {}, {}, {})", w, x, y, z);

    // // add y to z
    // *z += *y;
    println!("f: ({}, {}, {}, {})", w, x, y, z);
}

fn program2<'a, I>(mut input: I) -> (i64, i64, i64, i64)
where
    I: Iterator<Item = &'a i64>,
{
    let (mut w, mut x, mut y, mut z) = (0, 0, 0, 0);
    let offsets = [
        (15, 9, 1),
        (11, 1, 1),
        (10, 11, 1),
        (12, 3, 1),
        (-11, 10, 26),
        (11, 5, 1),
        (14, 0, 1),
        (-6, 7, 26),
        (10, 9, 1),
        (-6, 15, 26),
        (-6, 4, 26),
        (-16, 10, 26),
        (-4, 4, 26),
        (-2, 9, 26),
    ];
    offsets.iter().for_each(|(x_offset, y_offset, z_offset)| {
        println!("Offsets are {}, {}, {}", x_offset, y_offset, z_offset);
        f(
            &mut w,
            &mut x,
            &mut y,
            &mut z,
            *input.next().unwrap(),
            *x_offset,
            *y_offset,
            *z_offset,
        );
        println!();
    });
    (w, x, y, z)
}

fn program<'a, I>(mut input: I)
where
    I: Iterator<Item = &'a i64>,
{
    let (mut w, mut x, mut y, mut z) = (0, 0, 0, 0);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 15;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 9;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 1;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 10;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 11;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 12;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 3;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 10;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 5;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 14;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 0;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -6;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 7;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 10;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 9;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -6;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 15;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -6;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 4;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -16;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 10;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -4;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 4;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
    w = *input.next().unwrap();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -2;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 9;
    y *= x;
    z += y;

    println!("w: {}, x: {}, y: {}, z: {}", w, x, y, z);
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

    if let Some(input_str) = env::args().nth(2) {
        let input_nums = input_str
            .chars()
            .map(|c| {
                let i = c.to_digit(10).unwrap() as i64;
                if i == 0 || i > 9 {
                    panic!("Input number digits must be between 1 and 9")
                } else {
                    i
                }
            })
            .collect_vec();

        // println!("Instructions: {:?}", instructions);
        // program(input_nums.iter());
        program2(input_nums.iter());
        // let mut executor = Executor::new(&instructions);
        // executor.process(input_nums.iter())?;

        // executor.print_registry();
    } else {
        let l = largest(&instructions, Registry::new(0,0,0,0), 0, &mut HashMap::new());
        println!("{:?}", l);
        // for i in 0..14 {
        //     let mut registries = (1..=9).map(|j| {
        //         let mut input_nums = "20934501239009234509"
        //             .chars()
        //             .map(|c| c.to_digit(10).unwrap() as i64)
        //             .collect_vec();
        //         input_nums[i] = j;
        //         let mut executor = Executor::new(&instructions);
        //         executor.process(input_nums.iter());
        //         executor.registry
        //     });

        //     if registries.clone().all_equal() {
        //         println!(
        //             "All equal for index {}\n{:?}\n",
        //             i,
        //             registries.next().unwrap()
        //         );
        //     } else {
        //         println!("All not equal for index {}\n{:?}", i, registries);
        //     }
        // }
    }
    Ok(())
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Copy)]
struct Registry {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Registry {
    fn new(w: i64, x: i64, y: i64, z: i64) -> Registry {
        Registry { w, x, y, z }
    }

    fn at(&mut self, c: &char) -> Option<&mut i64> {
        match c {
            'w' => Some(&mut self.w),
            'x' => Some(&mut self.x),
            'y' => Some(&mut self.y),
            'z' => Some(&mut self.z),
            _ => None,
        }
    }

    fn get_target(&mut self, b: &Target) -> i64 {
        match b {
            Target::Number(n) => *n,
            Target::Variable(c) => *self.at(c).unwrap(),
        }
    }
}

fn largest(
    instructions: &[Instruction],
    registry: Registry,
    index: usize,
    visited: &mut HashMap<(Registry, usize), Option<i64>>,
) -> Option<i64> {
    if let Some(answer) = visited.get(&(registry, index)) {
        return *answer;
    }

    let range = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    'inputs: for input in range {
        let mut reg = registry;
        let mut index = index;
        instructions[index].process(&mut reg, input);
        index+= 1;

        while let Some(inst) = instructions.get(index) {
            if matches!(instructions[index], Instruction::Input(_)) {
                if let Some(best) = largest(instructions, reg, index, visited) {
                    visited.insert((reg, index), Some(best * 10 + input));
                    return Some(best * 10 + input);
                } else {
                    continue 'inputs;
                }
            } else {
                inst.process(&mut reg, input);
                index+= 1;
            }
        }

        if reg.z == 0 {
            visited.insert((reg, index), Some(input));
            return Some(input);
        }
    }

    visited.insert((registry, index), None);
    None
}
