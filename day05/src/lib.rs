use std::fs;

// each column is a stack
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2
fn read_input(file: &str) -> (Vec<Vec<char>>, Vec<Instr>) {
    // read each vertical line and add the numbers to a stack

    let input = fs::read_to_string(file).unwrap();
    //split at double newline
    let mut parts = input.split("\n\n");

    let stacks_input = parts.next().unwrap();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in stacks_input.lines() {
        // remove first char
        let chars = line.chars().skip(1);
        // save every 4th char in its own stack
        for (i, c) in chars.enumerate() {
            if i % 4 == 0 && stacks.len() <= i / 4 {
                stacks.push(Vec::new());
            }
            if i % 4 == 0 && c != ' ' && !c.is_numeric() {
                stacks[i / 4].insert(0, c);
            }
        }
    }

    let instructions_input = parts.next().unwrap();
    let mut instructions: Vec<Instr> = Vec::new();
    for line in instructions_input.lines() {
        let mut parts = line.split(' ');
        let _ = parts.next().unwrap();
        let amount = parts.next().unwrap().parse::<usize>().unwrap();
        let _ = parts.next().unwrap();
        let from = parts.next().unwrap().parse::<usize>().unwrap();
        let _ = parts.next().unwrap();
        let to = parts.next().unwrap().parse::<usize>().unwrap();
        instructions.push(Instr { amount, from, to });
    }
    (stacks, instructions)
}

struct Instr {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn part1() {
    let (mut stacks, instructions) = read_input("input/day05.in");
    for instr in instructions {
        let mut amount = instr.amount;
        while amount > 0 {
            if let Some(c) = stacks[instr.from - 1].pop() {
                stacks[instr.to - 1].push(c);
                amount -= 1;
            }
        }
    }

    //take the top char from each stack
    let mut result = String::new();
    for stack in stacks {
        result.push(stack[stack.len() - 1]);
    }
    println!("Result: {}", result);
}

pub fn part2() {
    // same as part1, but all chars move in the same order they were in
    let (mut stacks, instructions) = read_input("input/day05.in");
    for instr in instructions {
        //take top amount of chars from from stack
        let mut chars: Vec<char> = Vec::new();
        for _ in 0..instr.amount {
            if let Some(c) = stacks[instr.from - 1].pop() {
                chars.push(c);
            }
        }
        //put them on the to stack
        for c in chars.iter().rev() {
            stacks[instr.to - 1].push(*c);
        }
    }

    //take the top char from each stack
    let mut result = String::new();
    for stack in stacks {
        result.push(stack[stack.len() - 1]);
    }
    println!("Result: {}", result);
}
