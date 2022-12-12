use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

type Point = (usize, usize);

#[derive(Debug)]
struct Grid {
    start: Point,
    end: Point,
    grid: Vec<Vec<u32>>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut start = None;
        let mut end = None;
        for line in s.lines() {
            let mut row = Vec::new();

            for c in line.chars() {
                // a is lowest elevation, z is highest
                // S is start, E is end
                let height = match c {
                    'S' => {
                        start = Some((row.len(), grid.len()));
                        0
                    }
                    'E' => {
                        end = Some((row.len(), grid.len()));
                        'z' as u32 - 'a' as u32
                    }
                    'a'..='z' => c as u32 - 'a' as u32,
                    _ => return Err(()),
                };
                row.push(height);
            }
            grid.push(row);
        }
        Ok(Grid {
            start: start.unwrap(),
            end: end.unwrap(),
            grid,
        })
    }
}

impl Grid {
    // we can only transition to neighbours that are
    // one up or down in height from our current point
    fn get_possible_neighbours(&self, point: Point) -> Vec<Point> {
        let (x, y) = point;
        let height = self.grid[y][x];

        // up, right, down, left
        // make sure we don't subtract from 0
        let neighbours = vec![
            (x, y.saturating_sub(1)),
            (x + 1, y),
            (x, y + 1),
            (x.saturating_sub(1), y),
        ];

        let neighbours: Vec<Point> = neighbours
            .iter()
            .filter(|(x, y)| *x < self.grid[0].len() && *y < self.grid.len())
            // filter out neighbours that are the same as point
            .filter(|(x, y)| *x != point.0 || *y != point.1)
            .filter(|(x, y)| {
                let neighbour_height = self.grid[*y][*x];
                neighbour_height <= height + 1
            })
            .map(|(x, y)| (*x, *y))
            .collect();
        neighbours
    }

    fn bfs(&self, from: Point, to: Point) -> Option<usize> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((from, 0));
        visited.insert(from);
        while let Some((point, distance)) = queue.pop_front() {
            if point == to {
                return Some(distance);
            }
            let neighbours = self.get_possible_neighbours(point);
            for neighbour in neighbours {
                if !visited.contains(&neighbour) {
                    visited.insert(neighbour);
                    queue.push_back((neighbour, distance + 1));
                }
            }
        }
        None
    }
}

fn read_input(file: &str) -> Grid {
    std::fs::read_to_string(file).unwrap().parse().unwrap()
}

pub fn part1() {
    let grid = read_input("input/day12.in");
    println!("{:?}", grid);

    let path_len = grid.bfs(grid.start, grid.end);

    println!("Part 1: {}", path_len.unwrap_or(std::usize::MAX));
}

pub fn part2() {
    let grid = read_input("input/day12.in");

    //find shortest path from any point to end
    let mut paths = Vec::new();
    for i in 0..grid.grid.len() {
        for j in 0..grid.grid[i].len() {
            if grid.grid[i][j] != 0 {
                continue;
            }
            let path_len = grid.bfs((j, i), grid.end).unwrap_or(std::usize::MAX);
            paths.push(path_len);
        }
    }
    println!("Part 2: {}", paths.iter().min().unwrap());
}
