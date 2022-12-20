use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;
use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Blueprint {
    ore_robot: (u32, u32, u32),
    clay_robot: (u32, u32, u32),
    obsidian_robot: (u32, u32, u32),
    geode_robot: (u32, u32, u32),
}

impl Blueprint {
    fn max_cost(&self) -> (u32, u32, u32) {
        let ore = self.ore_robot.0.max(
            self.clay_robot
                .0
                .max(self.obsidian_robot.0.max(self.geode_robot.0)),
        );
        let clay = self.ore_robot.1.max(
            self.clay_robot
                .1
                .max(self.obsidian_robot.1.max(self.geode_robot.1)),
        );
        let obsidian = self.ore_robot.2.max(
            self.clay_robot
                .2
                .max(self.obsidian_robot.2.max(self.geode_robot.2)),
        );
        (ore, clay, obsidian)
    }
}

impl FromStr for Blueprint {
    type Err = String;

    //Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 3 ore and 8 obsidian.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blueprint = s.split(':').skip(1).next().unwrap();
        let mut ore_robot = (0, 0, 0);
        let mut clay_robot = (0, 0, 0);
        let mut obsidian_robot = (0, 0, 0);
        let mut geode_robot = (0, 0, 0);

        let find_amount = |resource: &str, parts: &Vec<&str>| -> u32 {
            if let Some(index) = parts.iter().position(|&x| x == resource) {
                return parts[index - 1].parse().unwrap();
            }
            0
        };

        for b in blueprint.split('.').take(4) {
            let resource = b
                .split_whitespace()
                .find(|&x| x == "ore" || x == "clay" || x == "obsidian" || x == "geode")
                .unwrap();
            let costs = b
                .split("costs")
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();
            let ore = find_amount("ore", &costs);
            let clay = find_amount("clay", &costs);
            let obsidian = find_amount("obsidian", &costs);
            match resource {
                "ore" => ore_robot = (ore, clay, obsidian),
                "clay" => clay_robot = (ore, clay, obsidian),
                "obsidian" => obsidian_robot = (ore, clay, obsidian),
                "geode" => geode_robot = (ore, clay, obsidian),
                _ => unreachable!(),
            }
        }

        Ok(Self {
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Factory {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl Factory {
    fn collect(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }

    fn start_building(&mut self, resource: &str, blueprint: &Blueprint) {
        match resource {
            "ore" => {
                self.ore -= blueprint.ore_robot.0;
                self.clay -= blueprint.ore_robot.1;
                self.obsidian -= blueprint.ore_robot.2;
            }
            "clay" => {
                self.ore -= blueprint.clay_robot.0;
                self.clay -= blueprint.clay_robot.1;
                self.obsidian -= blueprint.clay_robot.2;
            }
            "obsidian" => {
                self.ore -= blueprint.obsidian_robot.0;
                self.clay -= blueprint.obsidian_robot.1;
                self.obsidian -= blueprint.obsidian_robot.2;
            }
            "geode" => {
                self.ore -= blueprint.geode_robot.0;
                self.clay -= blueprint.geode_robot.1;
                self.obsidian -= blueprint.geode_robot.2;
            }
            _ => unreachable!(),
        }
    }

    fn finish_building(&mut self, resource: &str) {
        match resource {
            "ore" => self.ore_robots += 1,
            "clay" => self.clay_robots += 1,
            "obsidian" => self.obsidian_robots += 1,
            "geode" => self.geode_robots += 1,
            _ => unreachable!(),
        }
    }
}

fn find_maximum_geodes(blueprint: &Blueprint, time: u32) -> u32 {
    let mut queue: VecDeque<(u32, Factory)> = VecDeque::new();
    let mut visited: HashSet<Factory> = HashSet::new();
    let mut max_at_t: HashMap<u32, u32> = HashMap::new();
    let max_costs: (u32, u32, u32) = blueprint.max_cost();
    let time_remaining = time;
    let factory = Factory {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
    };

    queue.push_back((time_remaining, factory));
    while let Some((time_remaining, factory)) = queue.pop_front() {
        // println!("{} {:?}", time_remaining, factory);

        let potential_geodes = factory.geodes       // the geodes we already have
            + factory.geode_robots * time_remaining      // the geodes we can make with the robots we have
            + time_remaining * (time_remaining + 1) / 2; // the geodes we can make with the robots we can make, if we make one every minute
        let max = max_at_t.entry(time_remaining).or_insert(0);
        if *max > potential_geodes {
            continue;
        }
        if factory.geodes > *max {
            *max = factory.geodes;
        }

        if time_remaining <= 0 || visited.contains(&factory) {
            continue;
        }

        visited.insert(factory);

        if factory.ore >= blueprint.geode_robot.0
            && factory.clay >= blueprint.geode_robot.1
            && factory.obsidian >= blueprint.geode_robot.2
        {
            let mut new_factory = factory.clone();
            new_factory.start_building("geode", blueprint);
            new_factory.collect();
            new_factory.finish_building("geode");
            queue.push_back((time_remaining - 1, new_factory));
            continue;
        }

        if factory.ore >= blueprint.obsidian_robot.0
            && factory.clay >= blueprint.obsidian_robot.1
            && factory.obsidian >= blueprint.obsidian_robot.2
            && factory.obsidian_robots < max_costs.2
        {
            let mut new_factory = factory.clone();
            new_factory.start_building("obsidian", blueprint);
            new_factory.collect();
            new_factory.finish_building("obsidian");
            queue.push_back((time_remaining - 1, new_factory));
        }

        if factory.ore >= blueprint.clay_robot.0
            && factory.clay >= blueprint.clay_robot.1
            && factory.obsidian >= blueprint.clay_robot.2
            && factory.clay_robots < max_costs.1
        {
            let mut new_factory = factory.clone();
            new_factory.start_building("clay", blueprint);
            new_factory.collect();
            new_factory.finish_building("clay");
            queue.push_back((time_remaining - 1, new_factory));
        }

        if factory.ore >= blueprint.ore_robot.0
            && factory.clay >= blueprint.ore_robot.1
            && factory.obsidian >= blueprint.ore_robot.2
            && factory.ore_robots < max_costs.0
        {
            let mut new_factory = factory.clone();
            new_factory.start_building("ore", blueprint);
            new_factory.collect();
            new_factory.finish_building("ore");
            queue.push_back((time_remaining - 1, new_factory));
        }

        let mut new_factory = factory.clone();
        new_factory.collect();
        queue.push_back((time_remaining - 1, new_factory));
    }

    *max_at_t.entry(0).or_insert(0)
}

fn read_input(file: &str) -> Vec<Blueprint> {
    let input = std::fs::read_to_string(file).unwrap();
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1() {
    let blueprints = read_input("input/day19.in");

    let quality_total = blueprints
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let current_max = find_maximum_geodes(blueprint, 24);
            println!("Max geodes with blueprint {}: {}", i + 1, current_max);
            current_max * (i as u32 + 1)
        })
        .sum::<u32>();

    println!("Quality: {}", quality_total);
}
pub fn part2() {
    let blueprints = read_input("input/day19.in");

    let quality_total = blueprints
        .par_iter()
        .take(3)
        .enumerate()
        .map(|(i, blueprint)| {
            let current_max = find_maximum_geodes(blueprint, 32);
            println!("Max geodes with blueprint {}: {}", i + 1, current_max);
            current_max
        })
        .product::<u32>();

    println!("Quality: {}", quality_total);
}
