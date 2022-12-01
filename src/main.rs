use chrono::{Datelike, NaiveDate, Weekday};
use inquire::DateSelect;

fn main() {
    let problems = vec![
        (&day01::part1, &day01::part2),
        // (&day02::part1, &day02::part2),
        // (&day03::part1, &day03::part2),
        // (&day04::part1, &day04::part2),
        // (&day05::part1, &day05::part2),
        // (&day06::part1, &day06::part2),
        // (&day07::part1, &day07::part2),
        // (&day08::part1, &day08::part2),
        // (&day09::part1, &day09::part2),
        // (&day10::part1, &day10::part2),
        // (&day11::part1, &day11::part2),
        // (&day12::part1, &day12::part2),
        // (&day13::part1, &day13::part2),
        // (&day14::part1, &day14::part2),
        // (&day15::part1, &day15::part2),
        // (&day16::part1, &day16::part2),
        // (&day17::part1, &day17::part2),
        // (&day18::part1, &day18::part2),
        // (&day19::part1, &day19::part2),
        // (&day20::part1, &day20::part2),
        // (&day21::part1, &day21::part2),
        // (&day22::part1, &day22::part2),
        // (&day23::part1, &day23::part2),
        // (&day24::part1, &day24::part2),
        // (&day25::part1, &day25::part2),
    ];

    let date = DateSelect::new("Select a date")
        .with_default(NaiveDate::from_ymd_opt(2022, 12, 1).unwrap())
        .with_min_date(NaiveDate::from_ymd_opt(2022, 12, 1).unwrap())
        .with_max_date(NaiveDate::from_ymd_opt(2022, 12, 25).unwrap())
        .prompt();

    //parse into number 1-25
    let day: usize = date.unwrap().day() as usize;

    //get number 1 or 2
    let options = vec!["Part 1", "Part 2"];
    let part = inquire::Select::new("Select a part", options.clone())
        .prompt()
        .unwrap();

    //run the function
    let (part1, part2) = problems[day - 1];
    if part == options[0] {
        println!("Running Part 1 of Day {}", day);
        println!("========================");
        part1();
    } else if part == options[1] {
        println!("Running Part 2 of Day {}", day);
        println!("========================");
        part2();
    }
}
