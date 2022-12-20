use std::str::FromStr;

use pathfinding::{self, prelude::bfs_reach};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct RobotBlueprint {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl FromStr for RobotBlueprint {
    type Err = String;

    // Parses a string like:
    // "Each ore robot costs 4 ore"
    // or
    // "Each clay robot costs 4 ore"
    // or
    // "Each obsidian robot costs 3 ore and 11 clay"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // remove everything before "costs"
        // split on spaces
        let tokens = s
            .split("costs")
            .skip(1)
            .next()
            .unwrap()
            .split(' ')
            .collect::<Vec<_>>();

        // find the amount of a resource
        let find_amount = |tokens: &Vec<&str>, resource: &str| -> u32 {
            let amount = tokens
                .iter()
                .position(|&x| x == resource)
                .and_then(|i| tokens[i - 1].parse().ok())
                .unwrap_or(0);
            amount
        };

        let ore = find_amount(&tokens, "ore");
        let clay = find_amount(&tokens, "clay");
        let obsidian = find_amount(&tokens, "obsidian");
        let geode = find_amount(&tokens, "geode");

        Ok(RobotBlueprint {
            ore,
            clay,
            obsidian,
            geode,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct BlueprintCollection {
    ore_robot: RobotBlueprint,
    clay_robot: RobotBlueprint,
    obsidian_robot: RobotBlueprint,
    geode_robot: RobotBlueprint,
}

impl FromStr for BlueprintCollection {
    type Err = String;

    //Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 3 ore and 8 obsidian.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blueprint = s.split(':').skip(1).next().unwrap();
        let mut parts = blueprint.split('.').map(|s| s.trim());
        let ore = parts.next().unwrap().parse().unwrap();
        let clay = parts.next().unwrap().parse().unwrap();
        let obsidian = parts.next().unwrap().parse().unwrap();
        let geode = parts.next().unwrap().parse().unwrap();
        Ok(BlueprintCollection {
            ore_robot: ore,
            clay_robot: clay,
            obsidian_robot: obsidian,
            geode_robot: geode,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Factory {
    robots: Vec<Resource>,
    blueprint: BlueprintCollection,
    ore: u32,
    clay: u32,
    obisidan: u32,
    geode: u32,
    being_built: Option<Resource>,
}

impl Factory {
    fn collect_all(&mut self) {
        let robots = self.robots.clone();
        for robot in robots {
            self.collect(robot);
        }
    }

    fn collect(&mut self, resource: Resource) {
        match resource {
            Resource::Ore => {
                self.ore += 1;
            }
            Resource::Clay => {
                self.clay += 1;
            }
            Resource::Obsidian => {
                self.obisidan += 1;
            }
            Resource::Geode => {
                self.geode += 1;
            }
        }
    }

    // builds a robot if possible
    // if resources are missing, it will not build
    // if we have enough robots, it will not build
    fn build_robot(&mut self, resource: Resource) {
        if self.can_build_robot(resource) {
            self.start_building(resource);
        }
    }

    fn can_build_robot(&self, resource: Resource) -> bool {
        let blueprint = match resource {
            Resource::Ore => self.blueprint.ore_robot,
            Resource::Clay => self.blueprint.clay_robot,
            Resource::Obsidian => self.blueprint.obsidian_robot,
            Resource::Geode => self.blueprint.geode_robot,
        };
        if self.ore >= blueprint.ore
            && self.clay >= blueprint.clay
            && self.obisidan >= blueprint.obsidian
            && self.geode >= blueprint.geode
            && self.robots.iter().filter(|&r| *r == resource).count()
                < self.max_needed_robots(resource) as usize
        {
            true
        } else {
            false
        }
    }

    fn max_needed_robots(&self, resource: Resource) -> u32 {
        let max_ore_cost = self
            .blueprint
            .ore_robot
            .ore
            .max(self.blueprint.clay_robot.ore)
            .max(self.blueprint.obsidian_robot.ore)
            .max(self.blueprint.geode_robot.ore);
        let max_clay_cost = self
            .blueprint
            .ore_robot
            .clay
            .max(self.blueprint.clay_robot.clay)
            .max(self.blueprint.obsidian_robot.clay)
            .max(self.blueprint.geode_robot.clay);
        let max_obsidian_cost = self
            .blueprint
            .ore_robot
            .obsidian
            .max(self.blueprint.clay_robot.obsidian)
            .max(self.blueprint.obsidian_robot.obsidian)
            .max(self.blueprint.geode_robot.obsidian);
        let max_geode_cost = self
            .blueprint
            .ore_robot
            .geode
            .max(self.blueprint.clay_robot.geode)
            .max(self.blueprint.obsidian_robot.geode)
            .max(self.blueprint.geode_robot.geode);

        match resource {
            Resource::Ore => max_ore_cost,
            Resource::Clay => max_clay_cost,
            Resource::Obsidian => max_obsidian_cost,
            Resource::Geode => max_geode_cost,
        }
    }

    fn start_building(&mut self, resource: Resource) {
        let blueprint = match resource {
            Resource::Ore => self.blueprint.ore_robot,
            Resource::Clay => self.blueprint.clay_robot,
            Resource::Obsidian => self.blueprint.obsidian_robot,
            Resource::Geode => self.blueprint.geode_robot,
        };
        self.ore -= blueprint.ore;
        self.clay -= blueprint.clay;
        self.obisidan -= blueprint.obsidian;
        self.geode -= blueprint.geode;

        self.being_built = Some(resource);
    }

    fn finish_building(&mut self) {
        if let Some(resource) = self.being_built {
            self.robots.push(resource);
            self.being_built = None;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    factory: Factory,
    time_remaining: u32,
}

impl State {
    // the successors are all the possible actions that can be taken from this state
    // let the factory build a robot, or not
    // all the existing robots collect a resource
    fn successors(&self) -> Vec<State> {
        let mut successors = Vec::new();

        if self.time_remaining == 0 {
            return successors;
        }

        let mut next_factory = self.factory.clone();
        next_factory.finish_building();
        let time_remaining = self.time_remaining - 1;

        let mut new_robots = vec![];
        for resource in &[
            Resource::Ore,
            Resource::Clay,
            Resource::Obsidian,
            Resource::Geode,
        ] {
            let factory = next_factory.clone();
            if factory.can_build_robot(*resource) {
                new_robots.push(*resource);
            }
        }

        next_factory.collect_all();
        successors.push(State {
            factory: next_factory.clone(),
            time_remaining,
        });
        for robot in new_robots {
            let mut factory = next_factory.clone();
            factory.build_robot(robot);
            successors.push(State {
                factory,
                time_remaining,
            });
        }

        // println!("Successors: {:#?}", successors);
        successors
    }
}

fn read_input(file: &str) -> Vec<BlueprintCollection> {
    let input = std::fs::read_to_string(file).unwrap();
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1() {
    let blueprints = read_input("input/day19.example");
    //println!("{:#?}", blueprints);

    let mut max_geodes = 0;
    let mut max_geodes_blueprint = 1;
    for (i, blueprint) in blueprints.iter().enumerate() {
        // Start a factory with 1 ore robot
        let factory = Factory {
            robots: vec![Resource::Ore],
            blueprint: *blueprint,
            ore: 0,
            clay: 0,
            obisidan: 0,
            geode: 0,
            being_built: None,
        };

        let state = State {
            factory,
            time_remaining: 24,
        };

        let states = bfs_reach(state, |state| state.successors()).collect::<Vec<_>>();
        println!("Num states checked {}", states.len());

        let final_states = states
            .iter()
            .filter(|state| state.time_remaining == 0)
            .collect::<Vec<_>>();
        println!("Num final states {}", final_states.len());

        let max_geodes_in_final_states = final_states
            .iter()
            .map(|state| state.factory.geode)
            .max()
            .unwrap();

        println!(
            "Max geodes with blueprint {}: {}",
            i + 1,
            max_geodes_in_final_states
        );

        if max_geodes_in_final_states > max_geodes {
            max_geodes = max_geodes_in_final_states;
            max_geodes_blueprint = i + 1;
        }
    }
    println!("Max geodes: {}", max_geodes * max_geodes_blueprint as u32);
}

pub fn part2() {}
