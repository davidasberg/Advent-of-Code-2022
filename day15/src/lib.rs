use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    str::FromStr,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos(i64, i64);

impl Pos {
    fn manhattan_distance(&self, other: Pos) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    closest_beacon: Pos,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"

        let mut parts = s.split(':');
        let mut sensor = parts
            .next()
            .unwrap()
            .strip_prefix("Sensor at ")
            .unwrap()
            .split(',')
            .map(|s| s.trim());
        let x = sensor
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .parse()
            .unwrap();
        let y = sensor
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .parse()
            .unwrap();

        let mut beacon = parts
            .next()
            .unwrap()
            .strip_prefix(" closest beacon is at ")
            .unwrap()
            .split(',')
            .map(|s| s.trim());
        let bx = beacon
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .parse()
            .unwrap();
        let by = beacon
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .parse()
            .unwrap();

        Ok(Sensor {
            pos: Pos(x, y),
            closest_beacon: Pos(bx, by),
        })
    }
}

impl Sensor {
    // get all positions that we know are not beacons
    // since we know that the closest beacon is at (bx, by)
    // we can calculate all positions that are within the
    // distance of the closest beacon with manhattan distance

    fn coverage_at_line(&self, y: i64) -> Option<Range<i64>> {
        let y = (y - self.pos.1).abs();
        let half = self.pos.manhattan_distance(self.closest_beacon) - y;
        match half {
            _ if half < 0 => None,
            _ => Some(self.pos.0 - half..self.pos.0 + half + 1),
        }
    }
}

fn read_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|l| l.parse::<Sensor>().unwrap())
        .collect()
}

fn merge_ranges(ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
    let mut sorted = ranges;
    sorted.sort_by(|a, b| a.start.cmp(&b.start));
    let mut merged: Vec<Range<i64>> = Vec::new();
    for r in sorted {
        if let Some(last) = merged.last_mut() {
            if last.end >= r.start {
                *last = last.start.min(r.start)..last.end.max(r.end);
            } else {
                merged.push(r);
            }
        } else {
            merged.push(r);
        }
    }
    merged
}

pub fn part1() {
    let input = std::fs::read_to_string("input/day15.in").unwrap();
    let sensors = read_input(&input);
    let line = 2000000;
    let coverage_at_line = sensors
        .iter()
        .filter_map(|s| s.coverage_at_line(line))
        .collect::<Vec<_>>();

    let merged = merge_ranges(coverage_at_line);

    let num_beacons = sensors
        .iter()
        .filter(|s| s.closest_beacon.1 == line)
        .map(|s| s.closest_beacon)
        .collect::<HashSet<_>>()
        .len() as i64;

    let count = merged.iter().fold(0, |a, r| a + r.end - r.start);
    println!("Part 1: {}", count - num_beacons);
}

pub fn part2() {
    let input = std::fs::read_to_string("input/day15.in").unwrap();
    let bounds = 0..4000000 + 1;
    let sensors = read_input(&input);

    // for each y, get a vector of ranges that are covered by sensors
    let beacon = bounds
        .clone()
        .find_map(|y| {
            // get all ranges that are covered by sensors at y
            // and merge them
            let ranges = sensors
                .iter()
                .filter_map(|s| s.coverage_at_line(y))
                .collect::<Vec<_>>();
            let merged_ranges = merge_ranges(ranges);
            // println!("y: {}, merged: {:?}", y, merged_ranges);

            if merged_ranges.len() == 2 {
                return Some(Pos(merged_ranges[0].end, y));
            }
            None
        })
        .unwrap();
    println!("Pos: {:?}", beacon);
    let tuning_frequency = beacon.0 * 400000 + beacon.1;
    println!("Part 2: {}", tuning_frequency);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_ranges() {
        let ranges = vec![0..5, 2..7, 8..10, 11..12, 12..13, 14..15];
        let merged = merge_ranges(ranges);
        assert_eq!(merged, vec![0..7, 8..10, 11..13, 14..15]);
    }
}
