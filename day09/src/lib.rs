use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn dir_to_coord(dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

struct Move {
    direction: Direction,
    distance: u32,
}

type Point = (i32, i32);

fn read_input(file: &str) -> Vec<Move> {
    let input = std::fs::read_to_string(file).unwrap();
    let changes = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let dir = match parts.next() {
                Some("U") => Direction::Up,
                Some("D") => Direction::Down,
                Some("L") => Direction::Left,
                Some("R") => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let dist = parts.next().unwrap().parse::<u32>().unwrap();
            Move {
                direction: dir,
                distance: dist,
            }
        })
        .collect();
    changes
}

fn move_knot(knot: (i32, i32), direction: &Direction) -> Point {
    let (x, y) = knot;
    let change: (i32, i32) = dir_to_coord(direction);
    (x + change.0, y + change.1)
}

fn follow_knot(to_follow: Point, follower: Point) -> Point {
    let distance = (to_follow.0 - follower.0, to_follow.1 - follower.1);
    let adjacent = distance.0.abs() < 2 && distance.1.abs() < 2;
    if adjacent {
        return follower;
    }
    (
        follower.0 + distance.0.signum(),
        follower.1 + distance.1.signum(),
    )
}

fn solve<const NUM_KNOTS: usize>(moves: Vec<Move>) -> usize {
    let mut knots = [(0, 0); NUM_KNOTS];
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    for change in moves {
        let distance = change.distance;
        let direction = change.direction;
        for _ in 0..distance {
            knots[0] = move_knot(knots[0], &direction);
            (1..NUM_KNOTS).for_each(|i| knots[i] = follow_knot(knots[i - 1], knots[i]));
            tail_visited.insert(knots[NUM_KNOTS - 1]);
        }
    }
    tail_visited.len()
}

pub fn part1() {
    let moves = read_input("input/day09.in");
    println!("{}", solve::<2>(moves))
}

pub fn part2() {
    let moves = read_input("input/day09.in");
    println!("{}", solve::<10>(moves))
}
