use std::collections::HashSet;

type Pos = (i32, i32);

struct Cave {
    grid: HashSet<Pos>,  // positions that are occupied by rocks or sand
    ground: Option<i32>, // y coordinate of the ground
    sand_at_rest: usize,
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut min_x = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut min_y = std::i32::MAX;
        let mut max_y = std::i32::MIN;
        for (x, y) in &self.grid {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        s.push('\n');
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.grid.contains(&(x, y)) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Cave {
    fn from_lines(lines: Vec<Line>) -> Cave {
        let mut cave = Cave {
            grid: HashSet::new(),
            ground: None,
            sand_at_rest: 0,
        };
        for line in lines {
            cave.insert_line(line);
        }
        cave
    }

    fn insert_line(&mut self, line: Line) {
        let (start_x, start_y) = line.start;
        let (end_x, end_y) = line.end;
        if start_x == end_x {
            // vertical line
            for y in start_y..=end_y {
                self.grid.insert((start_x, y));
            }
        } else if start_y == end_y {
            // horizontal line
            for x in start_x..=end_x {
                self.grid.insert((x, start_y));
            }
        }
    }

    // sand drops down until it hits a something, then it goes down left, if it can't go down left, it goes down right
    // otherwise it stops
    // returns true if comes to rest
    // returns false if it falls indefinitely
    fn drop_sand(&mut self, pos: Pos) -> bool {
        let mut current_pos = pos;
        if self.grid.contains(&current_pos) {
            return false;
        }
        let mut lowest_point = self.grid.iter().map(|(_, y)| *y).max().unwrap();

        if self.ground.is_some() {
            lowest_point = self.ground.unwrap();
        }

        loop {
            if current_pos.1 > lowest_point {
                return false;
            }

            let is_below_ground = |pos: &Pos| -> bool {
                if let Some(ground) = self.ground {
                    pos.1 >= ground
                } else {
                    false
                }
            };

            // sand is falling
            let next_pos = (current_pos.0, current_pos.1 + 1);
            if !self.grid.contains(&next_pos) && !is_below_ground(&next_pos) {
                // can go down
                current_pos = next_pos;
                continue;
            }
            //sand has hit something

            // check if it can go left
            let left_pos = (current_pos.0 - 1, current_pos.1 + 1);
            if !self.grid.contains(&left_pos) && !is_below_ground(&left_pos) {
                // can go left
                current_pos = left_pos;
                continue;
            }
            // check if it can go right
            let right_pos = (current_pos.0 + 1, current_pos.1 + 1);
            if !self.grid.contains(&right_pos) && !is_below_ground(&right_pos) {
                // can go right
                current_pos = right_pos;
                continue;
            }
            // can't go down, left or right, so it's at rest
            self.sand_at_rest += 1;
            self.grid.insert(current_pos);
            break;
        }
        true
    }
}

#[derive(Debug)]
struct Line {
    start: Pos,
    end: Pos,
}

fn read_input(file: &str) -> Vec<Line> {
    let input = std::fs::read_to_string(file).unwrap();
    input
        .lines()
        .flat_map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            parts[..]
                .windows(2)
                .map(|pair| {
                    let start: Vec<i32> = pair[0].split(',').map(|s| s.parse().unwrap()).collect();
                    let end: Vec<i32> = pair[1].split(',').map(|s| s.parse().unwrap()).collect();
                    if start[0] == end[0] {
                        // vertical line
                        if start[1] > end[1] {
                            // swap
                            return Line {
                                start: (start[0], end[1]),
                                end: (end[0], start[1]),
                            };
                        }
                    } else if start[1] == end[1] {
                        // horizontal line
                        if start[0] > end[0] {
                            // swap
                            return Line {
                                start: (end[0], start[1]),
                                end: (start[0], end[1]),
                            };
                        }
                    }
                    Line {
                        start: (start[0], start[1]),
                        end: (end[0], end[1]),
                    }
                })
                .collect::<Vec<Line>>()
        })
        .collect()
}

pub fn part1() {
    let lines = read_input("input/day14.in");
    let mut cave = Cave::from_lines(lines);
    while cave.drop_sand((500, 0)) {}
    println!("Units of sand at rest: {}", cave.sand_at_rest);
}

pub fn part2() {
    let lines = read_input("input/day14.in");
    let mut cave = Cave::from_lines(lines);
    let lowest_point = cave.grid.iter().map(|(_, y)| y).max().unwrap();
    cave.ground = Some(*lowest_point + 2);
    while cave.drop_sand((500, 0)) {}
    println!("Units of sand at rest: {}", cave.sand_at_rest);
}
