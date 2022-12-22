use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum MonkeyExpr {
    Value(i128),
    Add(Box<MonkeyExpr>, Box<MonkeyExpr>),
    Sub(Box<MonkeyExpr>, Box<MonkeyExpr>),
    Mul(Box<MonkeyExpr>, Box<MonkeyExpr>),
    Div(Box<MonkeyExpr>, Box<MonkeyExpr>),
    Var(String),
}

impl std::fmt::Display for MonkeyExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonkeyExpr::Value(value) => write!(f, "{}", value),
            MonkeyExpr::Add(left, right) => write!(f, "({} + {})", left, right),
            MonkeyExpr::Sub(left, right) => write!(f, "({} - {})", left, right),
            MonkeyExpr::Mul(left, right) => write!(f, "({} * {})", left, right),
            MonkeyExpr::Div(left, right) => write!(f, "({} / {})", left, right),
            MonkeyExpr::Var(name) => write!(f, "{}", name),
        }
    }
}

impl MonkeyExpr {
    fn evaluate(&self, equalities: &HashMap<MonkeyExpr, MonkeyExpr>) -> MonkeyExpr {
        match self {
            MonkeyExpr::Value(value) => MonkeyExpr::Value(*value),
            MonkeyExpr::Add(left, right) => {
                let left = left.evaluate(equalities);
                let right = right.evaluate(equalities);
                match (left, right) {
                    (MonkeyExpr::Value(left), MonkeyExpr::Value(right)) => {
                        MonkeyExpr::Value(left + right)
                    }
                    (left, right) => MonkeyExpr::Add(Box::new(left), Box::new(right)),
                }
            }
            MonkeyExpr::Sub(left, right) => {
                let left = left.evaluate(equalities);
                let right = right.evaluate(equalities);
                match (left, right) {
                    (MonkeyExpr::Value(left), MonkeyExpr::Value(right)) => {
                        MonkeyExpr::Value(left - right)
                    }
                    (left, right) => MonkeyExpr::Sub(Box::new(left), Box::new(right)),
                }
            }
            MonkeyExpr::Mul(left, right) => {
                let left = left.evaluate(equalities);
                let right = right.evaluate(equalities);
                match (left, right) {
                    (MonkeyExpr::Value(left), MonkeyExpr::Value(right)) => {
                        MonkeyExpr::Value(left * right)
                    }
                    (left, right) => MonkeyExpr::Mul(Box::new(left), Box::new(right)),
                }
            }
            MonkeyExpr::Div(left, right) => {
                let left = left.evaluate(equalities);
                let right = right.evaluate(equalities);
                match (left, right) {
                    (MonkeyExpr::Value(left), MonkeyExpr::Value(right)) => {
                        MonkeyExpr::Value(left / right)
                    }
                    (left, right) => MonkeyExpr::Div(Box::new(left), Box::new(right)),
                }
            }
            MonkeyExpr::Var(var) => {
                let var = MonkeyExpr::Var(var.clone());
                if let Some(value) = equalities.get(&var) {
                    value.evaluate(equalities)
                } else {
                    var
                }
            }
        }
    }
}

fn read_input(file: &str) -> HashMap<MonkeyExpr, MonkeyExpr> {
    let input = std::fs::read_to_string(file).expect("Error reading input file!");
    input
        .lines()
        .map(|line| {
            // root: pppw + sjmn
            // dbpl: 5
            let mut parts = line.split(":");
            let monkey = parts.next().unwrap().to_string(); // root or dbpl

            let monkey_number = parts.next().unwrap().trim(); // pppw + sjmn or 5
            if let Ok(value) = monkey_number.parse::<i128>() {
                let value = MonkeyExpr::Value(value);
                let monkey = MonkeyExpr::Var(monkey);
                (monkey, value)
            } else {
                let mut parts = monkey_number.split(" ");
                let left = parts.next().unwrap().to_string();
                let op = parts.next().unwrap().to_string();
                let right = parts.next().unwrap().to_string();

                let left = MonkeyExpr::Var(left);
                let right = MonkeyExpr::Var(right);

                let monkey = MonkeyExpr::Var(monkey);
                match op.as_str() {
                    "+" => (monkey, MonkeyExpr::Add(Box::new(left), Box::new(right))),
                    "-" => (monkey, MonkeyExpr::Sub(Box::new(left), Box::new(right))),
                    "*" => (monkey, MonkeyExpr::Mul(Box::new(left), Box::new(right))),
                    "/" => (monkey, MonkeyExpr::Div(Box::new(left), Box::new(right))),
                    _ => panic!("Unknown op: {}", op),
                }
            }
        })
        .collect::<HashMap<_, _>>()
}

pub fn part1() {
    let monkeys = read_input("input/day21.in");
    let root = MonkeyExpr::Var("root".to_string());
    let root = root.evaluate(&monkeys);
    println!("{:?}", root);
}

pub fn part2() {
    let mut monkeys = read_input("input/day21.in");
    monkeys.remove(&MonkeyExpr::Var("humn".to_string()));
    let root = MonkeyExpr::Var("root".to_string());
    let expr = monkeys.get(&root).unwrap();
    let (mut left, right) = match expr {
        MonkeyExpr::Add(left, right)
        | MonkeyExpr::Sub(left, right)
        | MonkeyExpr::Mul(left, right)
        | MonkeyExpr::Div(left, right) => {
            let left = left.evaluate(&monkeys.clone());
            let right = right.evaluate(&monkeys.clone());
            (left, right)
        }
        _ => panic!("Unknown expr: {:?}", expr),
    };

    let mut right_value = match right {
        MonkeyExpr::Value(value) => value,
        _ => panic!("Right shoule be fully evalueated: {:?}", right),
    };

    println!("root: {} = {}", left, right_value);

    loop {
        match &left {
            MonkeyExpr::Add(inner_left, inner_right) => {
                let inner_left = inner_left.evaluate(&monkeys.clone());
                let inner_right = inner_right.evaluate(&monkeys.clone());
                if let MonkeyExpr::Value(value) = inner_left {
                    right_value -= value;
                    left = inner_right;
                } else if let MonkeyExpr::Value(value) = inner_right {
                    right_value -= value;
                    left = inner_left;
                } else {
                    unreachable!();
                }
            }
            MonkeyExpr::Sub(inner_left, inner_right) => {
                let inner_left = inner_left.evaluate(&monkeys.clone());
                let inner_right = inner_right.evaluate(&monkeys.clone());
                if let MonkeyExpr::Value(value) = inner_left {
                    right_value = value - right_value;
                    left = inner_right;
                } else if let MonkeyExpr::Value(value) = inner_right {
                    right_value += value;
                    left = inner_left;
                } else {
                    unreachable!();
                }
            }
            MonkeyExpr::Mul(inner_left, inner_right) => {
                let inner_left = inner_left.evaluate(&monkeys.clone());
                let inner_right = inner_right.evaluate(&monkeys.clone());
                if let MonkeyExpr::Value(value) = inner_left {
                    right_value /= value;
                    left = inner_right;
                } else if let MonkeyExpr::Value(value) = inner_right {
                    right_value /= value;
                    left = inner_left;
                } else {
                    unreachable!();
                }
            }
            MonkeyExpr::Div(inner_left, inner_right) => {
                let inner_left = inner_left.evaluate(&monkeys.clone());
                let inner_right = inner_right.evaluate(&monkeys.clone());
                if let MonkeyExpr::Value(value) = inner_left {
                    right_value = value / right_value;
                    left = inner_right;
                } else if let MonkeyExpr::Value(value) = inner_right {
                    right_value *= value;
                    left = inner_left;
                } else {
                    unreachable!();
                }
            }
            MonkeyExpr::Var(_) => {
                break;
            }
            _ => panic!("Unknown expr: {:?}", left),
        }

        println!("{} = {}", left, right_value);
    }
}
