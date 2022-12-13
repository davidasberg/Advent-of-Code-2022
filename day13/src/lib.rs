use std::str::FromStr;

use itertools::Itertools;

// list that can contain int or list
#[derive(Debug, PartialEq, Eq, Clone)]
enum List {
    Int(i32),
    List(Vec<List>),
    Empty,
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Int(i) => write!(f, "{}", i),
            List::List(l) => {
                write!(f, "[")?;
                for (i, item) in l.iter().enumerate() {
                    write!(f, "{}", item)?;
                    if i != l.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
            List::Empty => write!(f, ""),
        }
    }
}

impl FromStr for List {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim();

        if s.len() == 0 {
            return Ok(List::Empty);
        }

        if s.starts_with('[') && s.ends_with(']') {
            s = &s[1..];
        } else {
            return Err(());
        }

        let mut list = Vec::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '[' => {
                    // get the next "]"
                    let mut sub_list = String::new();
                    sub_list.push(c);
                    let mut depth = 1;
                    while let Some(c) = chars.next() {
                        sub_list.push(c);
                        if c == '[' {
                            depth += 1;
                        }
                        if c == ']' {
                            depth -= 1;
                        }
                        if depth == 0 {
                            break;
                        }
                    }
                    // convert to &str
                    let sub_list = &sub_list[..];
                    list.push(List::from_str(sub_list).unwrap());
                }
                ']' => {
                    break;
                }
                ',' => continue,
                _ => {
                    // parse into i32
                    let mut s = String::new();
                    s.push(c);
                    while chars.peek().is_some() && chars.peek().unwrap().is_digit(10) {
                        let c = chars.next().unwrap();
                        s.push(c);
                    }
                    let val = s.parse().unwrap();
                    list.push(List::Int(val));
                }
            }
        }
        Ok(List::List(list))
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (List::Int(a), List::Int(b)) => a.partial_cmp(b),
            (List::List(a), List::List(b)) => {
                for val in a.iter().zip_longest(b.iter()) {
                    match val {
                        itertools::EitherOrBoth::Both(_, _) => {}
                        itertools::EitherOrBoth::Left(_) => {
                            return Some(std::cmp::Ordering::Greater)
                        }
                        itertools::EitherOrBoth::Right(_) => return Some(std::cmp::Ordering::Less),
                    };
                    match a.partial_cmp(b) {
                        Some(std::cmp::Ordering::Equal) => {}
                        Some(std::cmp::Ordering::Less) => return Some(std::cmp::Ordering::Less),
                        Some(std::cmp::Ordering::Greater) => {
                            return Some(std::cmp::Ordering::Greater)
                        }
                        None => return None,
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
            (List::Int(a), List::List(_)) => {
                let temp = List::List(vec![List::Int(*a)]);
                temp.partial_cmp(other)
            }
            (List::List(_), List::Int(b)) => {
                let temp = List::List(vec![List::Int(*b)]);
                self.partial_cmp(&temp)
            }

            _ => None,
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn read_input(file: &str) -> Vec<List> {
    let input = std::fs::read_to_string(file).unwrap();
    let input = input.trim();
    let mut input = input.split("\n\n");

    let mut list = Vec::new();
    while let Some(pairs) = input.next() {
        let pairs = pairs.trim();
        let mut pairs = pairs.lines();
        let list1 = pairs.next().unwrap().parse().unwrap();
        let list2 = pairs.next().unwrap().parse().unwrap();
        list.extend([list1, list2]);
    }
    list
}

pub fn part1() {
    //for each pair, determine if they are in the right order
    let list = read_input("input/day13.in");

    let mut right_order = vec![];
    for (i, (list1, list2)) in list.iter().tuples().enumerate() {
        if list1 <= list2 {
            right_order.push(i + 1);
        }
    }
    println!("Part 1: {:?}", right_order.iter().sum::<usize>());
}

pub fn part2() {
    let mut list = read_input("input/day13.in");
    let divider1: List = "[[2]]]".parse().unwrap();
    let divider2: List = "[[6]]]".parse().unwrap();
    list.push(divider1.clone());
    list.push(divider2.clone());
    list.sort();
    let index = list.iter().position(|x| x == &divider1).unwrap();
    let index2 = list.iter().position(|x| x == &divider2).unwrap();

    println!("Part 2: {}", (index + 1) * (index2 + 1));
}
