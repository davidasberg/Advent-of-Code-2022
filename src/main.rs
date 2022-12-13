use chrono::{Datelike, NaiveDate};
use inquire::DateSelect;

fn main() {
    let problems = vec![
        (day01::part1 as fn(), day01::part2 as fn()),
        (day02::part1 as fn(), day02::part2 as fn()),
        (day03::part1 as fn(), day03::part2 as fn()),
        (day04::part1 as fn(), day04::part2 as fn()),
        (day05::part1 as fn(), day05::part2 as fn()),
        (day06::part1 as fn(), day06::part2 as fn()),
        (day07::part1 as fn(), day07::part2 as fn()),
        (day08::part1 as fn(), day08::part2 as fn()),
        (day09::part1 as fn(), day09::part2 as fn()),
        (day10::part1 as fn(), day10::part2 as fn()),
        (day11::part1 as fn(), day11::part2 as fn()),
        (day12::part1 as fn(), day12::part2 as fn()),
        (day13::part1 as fn(), day13::part2 as fn()),
    ];

    let dec_01 = NaiveDate::from_ymd_opt(2022, 12, 1).unwrap();
    let today_time = chrono::offset::Local::now();
    let today_date =
        NaiveDate::from_ymd_opt(today_time.year(), today_time.month(), today_time.day())
            .unwrap_or(dec_01);

    let date = DateSelect::new("Select a date")
        .with_default(NaiveDate::from_ymd_opt(2022, 12, 1).unwrap())
        .with_min_date(NaiveDate::from_ymd_opt(2022, 12, 1).unwrap())
        .with_max_date(NaiveDate::from_ymd_opt(2022, 12, 25).unwrap())
        .with_default(today_date)
        .prompt();

    //parse into number 1-25
    let day: usize = date.unwrap().day() as usize;

    //get number 1 or 2
    let options = vec!["Part 1", "Part 2"];
    let part = inquire::Select::new("Select a part", options.clone())
        .prompt()
        .unwrap();

    //run the function
    if problems.len() < day {
        unimplemented!("Day {} not implemented yet", day);
    }

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

pub fn part2() {}
