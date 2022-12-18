use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};
const CHAMBER_WIDTH: usize = 7;
const DEFAULT_ROW: [bool; CHAMBER_WIDTH] = [false; CHAMBER_WIDTH];
const MAX_ROCKS_PART1: usize = 2022;
const MAX_ROCKS_PART2: usize = 1000000000000;
const ROCK_ORDER: [RockType; 5] = [
    RockType::Horizontal,
    RockType::Plus,
    RockType::Angled,
    RockType::Vertical,
    RockType::Square,
];

type Pos = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum RockType {
    Horizontal, // 4x1 horizontal
    Vertical,   // 1x4 vertical
    Plus,       // a cross that is 3 space across
    Angled,     // an L shape that is 3 spaces to the right, and 3 spaces up
    Square,     // 2x2 square
}

impl RockType {
    // get all the spaces that a rock would occupy at a given position
    fn get_spaces(&self, pos: Pos) -> Vec<Pos> {
        let x = pos.0;
        let y = pos.1;
        match self {
            RockType::Horizontal => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            RockType::Vertical => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            RockType::Plus => vec![
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x + 1, y + 2),
                (x + 2, y + 1),
            ],
            RockType::Angled => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            RockType::Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    Left,
    Right,
    Down,
}

struct Cave<'a, I>
where
    I: Iterator<Item = &'a Direction>,
{
    grid: HashMap<usize, [bool; CHAMBER_WIDTH]>,
    highest_rock: usize,
    winds: I,
}

impl<'a, I> Cave<'a, I>
where
    I: Iterator<Item = &'a Direction>,
{
    fn new(winds: I) -> Self {
        let mut grid = HashMap::new();
        grid.insert(0, [true; CHAMBER_WIDTH]);
        Self {
            grid,
            highest_rock: 0,
            winds,
        }
    }

    fn spawn_rock(&mut self, rock: RockType) {
        let mut pos: Pos = (2, self.highest_rock + 4);

        let mut down = false;
        // self.print(pos, rock);
        // self.print(pos, rock);
        loop {
            let dir = if !down {
                self.get_next_wind()
            } else {
                Direction::Down
            };
            match dir {
                Direction::Left => {
                    let new_pos = (pos.0.saturating_sub(1), pos.1);
                    if self.can_spawn(&rock.get_spaces(new_pos)) {
                        pos = new_pos;
                    }
                }
                Direction::Right => {
                    let new_pos = (pos.0 + 1, pos.1);
                    if self.can_spawn(&rock.get_spaces(new_pos)) {
                        pos = new_pos;
                    }
                }
                Direction::Down => {
                    let new_pos = (pos.0, pos.1.saturating_sub(1));
                    if self.can_spawn(&rock.get_spaces(new_pos)) {
                        pos = new_pos;
                    } else {
                        break;
                    }
                }
            }
            down = !down;
            // self.print(pos, rock);
        }

        let spaces = rock.get_spaces(pos);
        let max_y = spaces.iter().map(|(_, y)| y).max().unwrap();
        for space in spaces.iter() {
            let x = space.0;
            let y = space.1;
            let row = self.grid.entry(y).or_insert(DEFAULT_ROW);
            row[x] = true;
        }
        self.highest_rock = self.highest_rock.max(*max_y);
    }

    fn can_spawn(&self, spaces: &[Pos]) -> bool {
        for space in spaces {
            let x = space.0;
            let y = space.1;
            if x >= CHAMBER_WIDTH {
                return false;
            }
            if let Some(row) = self.grid.get(&y) {
                if row[x] {
                    return false;
                }
            }
        }
        true
    }

    fn get_next_wind(&mut self) -> Direction {
        self.winds.next().unwrap().clone()
    }

    fn get_profile(&self, x: usize) -> usize {
        for (i, y) in (0..=self.highest_rock).rev().enumerate() {
            if let Some(row) = self.grid.get(&y) {
                if row[x] {
                    return i;
                }
            }
        }
        0
    }

    fn print(&self, pos: Pos, rock: RockType) {
        let spaces = rock.get_spaces(pos);
        let spaces: HashSet<(usize, usize)> = spaces.iter().copied().collect();
        for y in (1..=self.highest_rock + 4).rev() {
            let row = self.grid.get(&y).unwrap_or(&DEFAULT_ROW);
            print!("|");
            for x in 0..CHAMBER_WIDTH {
                if spaces.contains(&(x, y)) {
                    print!("@");
                } else if row[x] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("+-------+");
        println!();
    }

    fn copy_section(&mut self, copy_range: Range<usize>, to_y: usize) {
        for (i, y) in copy_range.clone().rev().enumerate() {
            let row = self.grid.entry(y).or_insert(DEFAULT_ROW).clone();
            let new_row = self.grid.entry(to_y - i).or_insert(DEFAULT_ROW);
            for x in 0..CHAMBER_WIDTH {
                new_row[x] = row[x];
            }
        }
        self.highest_rock = to_y;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    profile: [usize; CHAMBER_WIDTH],
    winds: Vec<Direction>,
    rock_type: RockType,
}

fn read_input(file: &str) -> Vec<Direction> {
    let input = std::fs::read_to_string(file).unwrap();
    let mut winds = Vec::new();
    for char in input.trim().chars() {
        let wind = match char {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid wind direction"),
        };
        winds.push(wind);
    }
    // println!("winds: {:?}", winds);
    winds
}

pub fn part1() {
    let winds = read_input("input/day17.example");
    let winds_cycle = winds.iter().cycle();
    let mut cave = Cave::new(winds_cycle);
    for (i, rock_type) in ROCK_ORDER.iter().cycle().enumerate() {
        println!("rock {}: {:?}", i, rock_type);
        if i >= MAX_ROCKS_PART1 {
            break;
        }
        cave.spawn_rock(*rock_type);
    }

    println!("Tallest rock: {}", cave.highest_rock);
}

pub fn part2() {
    let time = std::time::Instant::now();
    let winds = read_input("input/day17.in");
    let winds_cycle = winds.iter().cycle();
    let mut cave = Cave::new(winds_cycle);

    let mut seen_states: HashMap<State, (usize, usize)> = HashMap::new();
    let mut rock_order = ROCK_ORDER.iter().cycle();

    let mut drops = 0;
    while let Some(rock_type) = rock_order.next() {
        // println!("rock {}: {:?}", i, rock_type);
        if drops >= MAX_ROCKS_PART2 {
            break;
        }

        cave.spawn_rock(*rock_type);
        drops += 1;

        let profile: Vec<_> = (0..CHAMBER_WIDTH).map(|x| cave.get_profile(x)).collect();

        let state = State {
            profile: profile.try_into().unwrap(),
            winds: cave.winds.clone().copied().take(winds.len()).collect(),
            rock_type: *rock_type,
        };

        if let Some((prev_drops, prev_y)) = seen_states.insert(state, (drops, cave.highest_rock)) {
            let cycle_len = drops - prev_drops;
            println!(
                "Found cycle of length {} at {} drops, previous state at {} drops",
                cycle_len, drops, prev_drops
            );
            let cycles = (MAX_ROCKS_PART2 - drops) / cycle_len;
            println!("{} cycles", cycles);
            drops += cycles * cycle_len;
            let copy_range = prev_y + 1..cave.highest_rock + 1;
            println!("Copying range {:?}", copy_range);
            let to_y = cave.highest_rock + (cave.highest_rock - prev_y) * cycles;
            println!("Copying to {}", to_y);
            cave.copy_section(copy_range, to_y);
            seen_states.clear();
        }
    }

    let elapsed = time.elapsed();
    println!("Tallest rock: {}", cave.highest_rock);
    println!("Time: {}ms", elapsed.as_millis());
}
