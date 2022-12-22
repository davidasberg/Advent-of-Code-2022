use std::collections::HashMap;

type Monkey = String;

#[derive(Debug)]
enum MonkeyNumber {
    Value(i64),
    Add(Monkey, Monkey),
    Sub(Monkey, Monkey),
    Mul(Monkey, Monkey),
    Div(Monkey, Monkey),
}

impl MonkeyNumber {
    fn value(&self, monkeys: &HashMap<Monkey, MonkeyNumber>) -> i64 {
        println!("Evaluating {:?}", self);
        match self {
            MonkeyNumber::Value(value) => *value,
            MonkeyNumber::Add(monkey1, monkey2) => {
                monkeys[monkey1].value(monkeys) + monkeys[monkey2].value(monkeys)
            }
            MonkeyNumber::Sub(monkey1, monkey2) => {
                monkeys[monkey1].value(monkeys) - monkeys[monkey2].value(monkeys)
            }
            MonkeyNumber::Mul(monkey1, monkey2) => {
                monkeys[monkey1].value(monkeys) * monkeys[monkey2].value(monkeys)
            }
            MonkeyNumber::Div(monkey1, monkey2) => {
                monkeys[monkey1].value(monkeys) / monkeys[monkey2].value(monkeys)
            }
        }
    }
}

fn read_input(file: &str) -> HashMap<Monkey, MonkeyNumber> {
    let input = std::fs::read_to_string(file).expect("Error reading input file!");
    input
        .lines()
        .map(|line| {
            // root: pppw + sjmn
            // dbpl: 5
            let mut parts = line.split(":");
            let monkey = parts.next().unwrap().to_string(); // root or dbpl

            let monkey_number = parts.next().unwrap().trim(); // pppw + sjmn or 5
            if let Ok(value) = monkey_number.parse::<i64>() {
                return (monkey, MonkeyNumber::Value(value));
            } else {
                let mut parts = monkey_number.split_whitespace();
                let monkey1 = parts.next().unwrap().to_string();
                let op = parts.next().unwrap().to_string();
                let monkey2 = parts.next().unwrap().to_string();
                match op.as_str() {
                    "+" => (monkey, MonkeyNumber::Add(monkey1, monkey2)),
                    "-" => (monkey, MonkeyNumber::Sub(monkey1, monkey2)),
                    "*" => (monkey, MonkeyNumber::Mul(monkey1, monkey2)),
                    "/" => (monkey, MonkeyNumber::Div(monkey1, monkey2)),
                    _ => panic!("Unknown operator: {}", op),
                }
            }
        })
        .collect::<HashMap<Monkey, MonkeyNumber>>()
}

pub fn part1() {
    let monkeys = read_input("input/day21.in");
    let root = monkeys.get("root").unwrap();
    let root_number = root.value(&monkeys);
    println!("root: {}", root_number);
}

pub fn part2() {}
