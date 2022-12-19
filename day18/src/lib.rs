use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    const NUM_SIDES: usize = 6;
    const SIDES: [Pos; Pos::NUM_SIDES] = [
        Pos { x: 1, y: 0, z: 0 },
        Pos { x: -1, y: 0, z: 0 },
        Pos { x: 0, y: 1, z: 0 },
        Pos { x: 0, y: -1, z: 0 },
        Pos { x: 0, y: 0, z: 1 },
        Pos { x: 0, y: 0, z: -1 },
    ];

    fn is_in_bounds(&self, lower_bounds: &Pos, upper_bounds: &Pos) -> bool {
        self.x >= lower_bounds.x
            && self.x <= upper_bounds.x
            && self.y >= lower_bounds.y
            && self.y <= upper_bounds.y
            && self.z >= lower_bounds.z
            && self.z <= upper_bounds.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LavaVoxel(Pos);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SteamVoxel(Pos);

// A droplet is represented by a 3D grid
#[derive(Debug, Clone)]
struct Volume {
    lava: HashSet<LavaVoxel>,
    steam: HashSet<SteamVoxel>,
}

impl FromStr for Volume {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashSet::new();
        for line in s.lines() {
            let point = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let voxel = LavaVoxel(Pos {
                x: point[0],
                y: point[1],
                z: point[2],
            });
            grid.insert(voxel);
        }

        Ok(Volume {
            lava: grid,
            steam: HashSet::new(),
        })
    }
}

impl Volume {
    fn get_air_neighbours(&self, point: &Pos) -> usize {
        let mut count = 0;
        for side in &Pos::SIDES {
            let neighbour = Pos {
                x: point.x + side.x,
                y: point.y + side.y,
                z: point.z + side.z,
            };
            if !self.lava.contains(&LavaVoxel(neighbour))
                && !self.steam.contains(&SteamVoxel(neighbour))
            {
                count += 1;
            }
        }
        count
    }

    fn get_steam_neighbours(&self, point: &Pos) -> usize {
        let mut count = 0;
        for side in &Pos::SIDES {
            let neighbour = Pos {
                x: point.x + side.x,
                y: point.y + side.y,
                z: point.z + side.z,
            };
            if self.steam.contains(&SteamVoxel(neighbour)) {
                count += 1;
            }
        }
        count
    }

    fn get_lava_surface(&self) -> usize {
        let mut surface_area = 0;
        for voxel in &self.lava {
            let empty_sides = self.get_air_neighbours(&voxel.0);
            let steam_neighbours = self.get_steam_neighbours(&voxel.0);
            surface_area += empty_sides + steam_neighbours;
        }
        surface_area
    }

    fn get_lava_exterior_surface(&self) -> usize {
        let mut surface_area = 0;
        for voxel in &self.lava {
            let empty_sides = self.get_steam_neighbours(&voxel.0);
            surface_area += empty_sides;
        }
        surface_area
    }

    fn get_lava_bounds(&self) -> (Pos, Pos) {
        let min_x = self.lava.iter().map(|x| x.0.x).min().unwrap();
        let min_y = self.lava.iter().map(|x| x.0.y).min().unwrap();
        let min_z = self.lava.iter().map(|x| x.0.z).min().unwrap();
        let max_x = self.lava.iter().map(|x| x.0.x).max().unwrap();
        let max_y = self.lava.iter().map(|x| x.0.y).max().unwrap();
        let max_z = self.lava.iter().map(|x| x.0.z).max().unwrap();

        let lower_bounds = Pos {
            x: min_x - 1,
            y: min_y - 1,
            z: min_z - 1,
        };

        let upper_bounds = Pos {
            x: max_x + 1,
            y: max_y + 1,
            z: max_z + 1,
        };

        (lower_bounds, upper_bounds)
    }

    fn expand_steam_from(&mut self, point: Pos, bounds: &mut (Pos, Pos)) {
        let lower_bounds = &mut bounds.0;
        let upper_bounds = &mut bounds.1;
        let mut queue = vec![point];
        while !queue.is_empty() {
            let current = queue.pop().unwrap();
            for side in &Pos::SIDES {
                let neighbour = Pos {
                    x: current.x + side.x,
                    y: current.y + side.y,
                    z: current.z + side.z,
                };

                // if neighbour is not in bounds, skip
                if !neighbour.is_in_bounds(lower_bounds, upper_bounds)
                    || self.lava.contains(&LavaVoxel(neighbour))
                {
                    continue;
                }

                if self.steam.insert(SteamVoxel(neighbour)) {
                    queue.push(neighbour);
                }
            }
        }
    }
}

// each line is one point in the grid, with 3 values, x, y, z
fn read_input(file: &str) -> Volume {
    let input = std::fs::read_to_string(file).unwrap();
    input.parse().unwrap()
}

pub fn part1() {
    let volume = read_input("input/day18.in");
    let surface_area = volume.get_lava_surface();
    println!("Part 1: {}", surface_area);
}

pub fn part2() {
    let mut volume = read_input("input/day18.in");

    let (lower_bounds, upper_bounds) = volume.get_lava_bounds();

    let mut bounds = (lower_bounds, upper_bounds);
    println!("{:?}", bounds);
    volume.expand_steam_from(lower_bounds, &mut bounds);
    let exterior_surface = volume.get_lava_exterior_surface();
    println!("{:?}", volume.lava.len());
    println!("{:?}", volume.steam.len());
    println!("Part 2: {}", exterior_surface);
}
