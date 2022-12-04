use std::fs;

struct Range {
    start: u32,
    end: u32,
}

fn read_input(file: &str) -> Vec<(Range, Range)> {
    let input: String = fs::read_to_string(file).expect("Error reading file");

    let mut ranges = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(',');
        let range1 = parts.next().unwrap();
        let range2 = parts.next().unwrap();
        let mut parts = range1.split('-');
        let start1 = parts.next().unwrap().parse::<u32>().unwrap();
        let end1 = parts.next().unwrap().parse::<u32>().unwrap();
        let mut parts = range2.split('-');
        let start2 = parts.next().unwrap().parse::<u32>().unwrap();
        let end2 = parts.next().unwrap().parse::<u32>().unwrap();
        ranges.push((
            Range {
                start: start1,
                end: end1,
            },
            Range {
                start: start2,
                end: end2,
            },
        ));
    }
    ranges
}

pub fn part1() {
    // count the number of pairs where one range fully contains the other
    let ranges = read_input("input/day04.in");
    let mut count = 0;
    for (range1, range2) in ranges.iter() {
        if (range1.start <= range2.start && range1.end >= range2.end)
            || (range2.start <= range1.start && range2.end >= range1.end)
        {
            count += 1;
        }
    }
    println!("{}", count);
}

pub fn part2() {
    // count the number of pairs where the ranges overlap
    let ranges = read_input("input/day04.in");
    let mut count = 0;
    for (range1, range2) in ranges.iter() {
        if (range1.start <= range2.start && range1.end >= range2.start)
            || (range2.start <= range1.start && range2.end >= range1.start)
        {
            count += 1;
        }
    }
    println!("{}", count);
}
