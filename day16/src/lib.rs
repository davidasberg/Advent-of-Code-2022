use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{line_ending, u32},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use itertools::Itertools;
const NUM_VALVES: usize = 55;

struct NamedValve<'a> {
    name: &'a str,
    flow: usize,
    tunnels: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    flow: usize,
    neighbours: Vec<usize>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Cave {
    aa_id: usize,
    valves: Vec<Valve>,
    name_to_id: HashMap<String, usize>,
}

impl Cave {
    fn calc_distances(&self) -> [[usize; NUM_VALVES]; NUM_VALVES] {
        let mut distances = [[usize::MAX; NUM_VALVES]; NUM_VALVES];
        let mut seen = HashSet::new();
        for (id, _) in self
            .valves
            .iter()
            .enumerate()
            .filter(|(id, v)| v.flow > 0 || *id == self.aa_id)
        {
            let mut current = HashSet::new();
            current.insert(id);
            let mut next = HashSet::new();
            let mut distance = 0;

            distances[id][id] = 0;
            while !current.is_empty() {
                distance += 1;
                for valve in &current {
                    for neighbour in &self.valves[*valve].neighbours {
                        if !seen.contains(&(id, *neighbour)) {
                            next.insert(*neighbour);
                            distances[id][*neighbour] = distance;
                            seen.insert((id, *neighbour));
                        }
                    }
                }
                current = next;
                next = HashSet::new();
            }
        }
        distances
    }
}

fn read_input(file: &str) -> Cave {
    let input = std::fs::read_to_string(file).unwrap();
    let (_, valves) = all_consuming(separated_list1(line_ending, parse_valve))(&input).unwrap();
    let mut name_to_id = HashMap::new();
    valves.iter().for_each(|valve| {
        let id = name_to_id.len();
        name_to_id.insert(valve.name.to_string(), id);
    });

    let valves = valves
        .iter()
        .map(|valve| {
            let neighbours = valve
                .tunnels
                .iter()
                .map(|name| *name_to_id.get(*name).unwrap())
                .collect();
            Valve {
                flow: valve.flow,
                neighbours,
            }
        })
        .collect();

    let aa_id = *name_to_id.get("AA").unwrap();
    Cave {
        aa_id,
        valves,
        name_to_id,
    }
}

fn parse_valve(input: &str) -> IResult<&str, NamedValve> {
    let (input, name) = preceded(tag("Valve "), take(2usize))(input)?;
    let (input, flow) = preceded(tag(" has flow rate="), u32)(input)?;
    let (input, tunnels) = preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), take(2usize)),
    )(input)?;
    Ok((
        input,
        NamedValve {
            name,
            flow: flow as usize,
            tunnels,
        },
    ))
}

fn find_max_release(
    distances: &[[usize; NUM_VALVES]; NUM_VALVES],
    cave: &Cave,
    valves_to_release: &mut HashSet<usize>,
    time: usize,
    current: usize,
) -> (usize, HashSet<usize>) {
    valves_to_release.remove(&current);
    let mut max = 0;
    let mut path = HashSet::new();
    for valve in valves_to_release.iter() {
        let time_remaining = time
            .saturating_sub(distances[current][*valve])
            .saturating_sub(1);
        // println!("{} -> {} = {}", current, valve, time_remaining);
        if time_remaining > 0 {
            let mut flow = cave.valves[*valve].flow * time_remaining;
            let (new_flow, mut new_path) = find_max_release(
                distances,
                cave,
                &mut valves_to_release.clone(),
                time_remaining,
                *valve,
            );
            flow += new_flow;
            if flow > max {
                max = flow;
                new_path.insert(current);
                path = new_path;
            }
        }
    }
    (max, path)
}

// After hours of trying to get it to work,
// I gave up and found someone who made a similar idea
// and changed mine to match theirs.
// https://github.com/synapticarbors/advent_of_code_2022/blob/main/rust/aoc16/src/main.rs#L61
// It was surprisingly similar to my original idea,
// but it worked..
pub fn part1() {
    let cave = read_input("input/day16.in");
    let distances = cave.calc_distances();

    // println!("{:#?}", cave);
    let mut valves_to_release = HashSet::from_iter(
        cave.valves
            .iter()
            .enumerate()
            .filter(|(id, v)| v.flow > 0 || *id == cave.aa_id)
            .map(|(id, _)| id),
    );
    // println!("{:#?}", valves_to_release);

    let (release, _) = find_max_release(&distances, &cave, &mut valves_to_release, 30, cave.aa_id);

    // println!("{:#?}", path);
    println!("Part 1: {}", release);
}

// this part is completely my own idea
pub fn part2() {
    let cave = read_input("input/day16.in");
    let distances = cave.calc_distances();

    let valves_to_release = cave
        .valves
        .iter()
        .enumerate()
        .filter(|(id, v)| v.flow > 0 || *id == cave.aa_id)
        .map(|(id, _)| id)
        .collect::<HashSet<_>>();

    // the most efficient way to release the valves is to split them into two groups
    // the human and the elephant will release the same number of valves
    // so we split the valves that need to be released into two groups
    // of equal size
    let mut all_groups = Vec::new();
    for group1 in valves_to_release
        .iter()
        .copied()
        .combinations(valves_to_release.len() / 2)
    {
        let group2: Vec<usize> = valves_to_release
            .difference(&HashSet::from_iter(group1.clone()))
            .cloned()
            .collect();
        all_groups.push((group1, group2));
    }

    // for every pair of groups, find the max release
    // if the release is greater than the current max, update the max
    let mut max = 0;
    for (group1, group2) in all_groups {
        let mut group1 = HashSet::from_iter(group1);
        let mut group2 = HashSet::from_iter(group2);
        let (release1, _) = find_max_release(&distances, &cave, &mut group1, 26, cave.aa_id);
        let (release2, _) = find_max_release(&distances, &cave, &mut group2, 26, cave.aa_id);
        let release = release1 + release2;
        if release > max {
            max = release;
        }
    }

    println!("Part 2: {}", max);
}
