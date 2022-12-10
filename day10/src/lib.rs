use std::collections::VecDeque;

enum Instruction {
    AddX(i32), // takes two cycles
    NoOp,      // takes one cycle
}

struct CRT {
    h: u32,
    w: u32,
    pixels: Vec<bool>,
}

impl CRT {
    fn new(h: u32, w: u32) -> CRT {
        CRT {
            h,
            w,
            pixels: vec![false; (h * w) as usize],
        }
    }

    fn set_pixel(&mut self, x: i32) {
        if x < 0 || x >= (self.w * self.h) as i32 {
            return;
        }
        self.pixels[x as usize] = true;
    }

    fn print(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                let pixel = self.pixels[(y * self.w + x) as usize];
                if pixel {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn read_input(file: &str) -> Vec<Instruction> {
    let input = std::fs::read_to_string(file).unwrap();
    let instr = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let instr = match parts.next() {
                Some("noop") => Instruction::NoOp,
                Some("addx") => Instruction::AddX(parts.next().unwrap().parse::<i32>().unwrap()),
                _ => panic!("Invalid instruction"),
            };
            instr
        })
        .collect();
    instr
}

pub fn part1() {
    let instructions = read_input("input/day10.in");
    let mut instructions = instructions.iter().peekable();
    let mut current_instr: (Instruction, u32) = (Instruction::NoOp, 0);
    let mut x = 1;
    let mut signal_strength_sum = 0;
    let mut c = 1;
    while instructions.peek().is_some() || current_instr.1 > 0 {
        if current_instr.1 == 0 {
            match instructions.next() {
                Some(Instruction::NoOp) => {
                    current_instr = (Instruction::NoOp, 1);
                }
                Some(Instruction::AddX(n)) => {
                    current_instr = (Instruction::AddX(*n), 2);
                }
                None => panic!("Ran out of instructions"),
            }
        }

        match c {
            20 | 60 | 100 | 140 | 180 | 220 => {
                signal_strength_sum += x * c;
                println!("{}: {}", c, x * c);
            }
            _ => {}
        }

        current_instr.1 -= 1;
        if current_instr.1 == 0 {
            match current_instr.0 {
                Instruction::NoOp => {}
                Instruction::AddX(n) => {
                    x += n;
                }
            }
        }

        println!("{}: {}", c, x);

        c += 1;
    }
    println!("Part 1: {}", signal_strength_sum);
}

pub fn part2() {
    let instructions = read_input("input/day10.in");
    let mut instructions = instructions.iter().peekable();
    let mut current_instr: (Instruction, u32) = (Instruction::NoOp, 0);
    let mut x = 1;
    let mut c = 1;
    let mut crt = CRT::new(6, 40);
    while instructions.peek().is_some() || current_instr.1 > 0 {
        if current_instr.1 == 0 {
            match instructions.next() {
                Some(Instruction::NoOp) => {
                    current_instr = (Instruction::NoOp, 1);
                }
                Some(Instruction::AddX(n)) => {
                    current_instr = (Instruction::AddX(*n), 2);
                    println!("Begin executing addx {} at cycle {}", n, c);
                }
                None => panic!("Ran out of instructions"),
            }
        }

        // draw pixel
        let pixel = (c - 1) % 40;
        if pixel >= x - 1 && pixel <= x + 1 {
            crt.set_pixel(c - 1);
            println!("During cycle {}: CRT drawing at {}", c, c - 1);
        }

        current_instr.1 -= 1;
        if current_instr.1 == 0 {
            match current_instr.0 {
                Instruction::NoOp => {}
                Instruction::AddX(n) => {
                    x += n;
                    println!("End of cycle {}: Register is now {}", c, x)
                }
            }
        }
        c += 1;
        println!();
    }
    crt.print();
}
