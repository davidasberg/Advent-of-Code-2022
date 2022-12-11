use std::{iter, str::FromStr};

#[derive(Copy, Clone)]
enum Operation {
    Add(i32),
    Mult(i32),
    Square,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "new = old * old" {
            Ok(Operation::Square)
        } else if s.contains("+") {
            let s = s.split(" ").collect::<Vec<&str>>();
            let s = s[s.len() - 1].parse::<i32>().unwrap();
            Ok(Operation::Add(s))
        } else {
            let s = s.split(" ").collect::<Vec<&str>>();
            let s = s[s.len() - 1].parse::<i32>().unwrap();
            Ok(Operation::Mult(s))
        }
    }
}

impl Operation {
    fn apply(&self, a: u64) -> u64 {
        match self {
            Operation::Add(b) => a + *b as u64,
            Operation::Mult(b) => a * *b as u64,
            Operation::Square => a * a,
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_test: u64,
    throw_true: usize,
    throw_false: usize,
}

fn read_input(input: &str) -> Option<Vec<Monkey>> {
    let input = std::fs::read_to_string(input).unwrap();
    let mut lines = input.lines().into_iter().peekable();
    let mut monkeys = Vec::new();
    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        if line.is_empty() {
            continue;
        }
        let items = lines
            .next()?
            .rsplit_once(":")?
            .1
            .split(",")
            .map(|x| x.trim().parse().ok())
            .collect::<Option<_>>()?;
        let operation = lines.next()?.rsplit_once(": ")?.1.parse().ok()?;
        let divisible_test = lines
            .next()?
            .split_whitespace()
            .rev()
            .next()?
            .parse()
            .ok()?;
        let throw_true = lines
            .next()?
            .split_whitespace()
            .rev()
            .next()?
            .parse()
            .ok()?;
        let throw_false = lines
            .next()?
            .split_whitespace()
            .rev()
            .next()?
            .parse()
            .ok()?;
        let monkey = Monkey {
            items,
            operation,
            divisible_test,
            throw_true,
            throw_false,
        };
        monkeys.push(monkey);
    }
    Some(monkeys)
}

pub fn part1() {
    let mut monkeys = read_input("input/day11.in").unwrap();
    let mut monkey_count = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let items = monkey.items.drain(..).collect::<Vec<_>>();
            let Monkey {
                operation,
                divisible_test,
                throw_true,
                throw_false,
                ..
            } = *monkey;
            monkey_count[i] += items.len();
            for item in items {
                let mut item = operation.apply(item);
                item /= 3;

                let other_monkey: usize = if item % divisible_test == 0 {
                    throw_true
                } else {
                    throw_false
                };
                monkeys[other_monkey].items.push(item);
            }
        }
    }
    monkey_count.sort();
    println!("{:?}", monkey_count);
    // multiply the last two
    let val = monkey_count.iter().rev().take(2).product::<usize>();
    println!("{}", val);
}

pub fn part2() {
    let mut monkeys = read_input("input/day11.in").unwrap();
    let modulo_divisor = monkeys
        .iter()
        .fold(1, |acc, x| acc * x.divisible_test as u64);
    let mut monkey_count = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let items = monkey.items.drain(..).collect::<Vec<_>>();
            let Monkey {
                operation,
                divisible_test,
                throw_true,
                throw_false,
                ..
            } = *monkey;
            monkey_count[i] += items.len();
            for item in items {
                let mut item = operation.apply(item);
                item = item % modulo_divisor;

                let other_monkey: usize = if item % divisible_test == 0 {
                    throw_true
                } else {
                    throw_false
                };
                monkeys[other_monkey].items.push(item);
            }
        }
    }
    monkey_count.sort();
    println!("{:?}", monkey_count);
    // multiply the last two
    let val = monkey_count.iter().rev().take(2).product::<usize>();
    println!("{}", val);
}
