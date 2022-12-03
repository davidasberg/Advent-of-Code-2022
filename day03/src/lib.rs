use std::fs;

fn read_input(file: &str) -> Vec<String> {
    let input: String = fs::read_to_string(file).expect("Error reading file");
    input.split('\n').map(|s| s.to_string()).collect()
}

pub fn part1() {
    let bags = read_input("input/day03.in");
    let mut sum = 0;
    for bag in bags {
        let len = bag.len();
        let first_half = &bag[0..len / 2];
        let second_half = &bag[len / 2..len];

        //find letter appears in both halves
        let mut common_char: char = ' ';
        for c in first_half.chars() {
            if second_half.contains(c) {
                common_char = c;
                break;
            }
        }

        // Lowercase item types a through z have priorities 1 through 26.
        // Uppercase item types A through Z have priorities 27 through 52.
        sum += match common_char {
            'a'..='z' => common_char as u32 - 'a' as u32 + 1,
            'A'..='Z' => common_char as u32 - 'A' as u32 + 27,
            _ => panic!("Invalid input"),
        };
    }

    println!("{}", sum);
}

pub fn part2() {
    let bags = read_input("input/day03.in");
    let mut sum = 0;
    //iterate over 3 bags at a time
    for i in (0..bags.len()).step_by(3) {
        let bag1 = &bags[i];
        let bag2 = &bags[i + 1];
        let bag3 = &bags[i + 2];

        //find letter appears in all 3 bags
        let mut common_char: char = ' ';
        for c in bag1.chars() {
            if bag2.contains(c) && bag3.contains(c) {
                common_char = c;
                break;
            }
        }

        // Lowercase item types a through z have priorities 1 through 26.
        // Uppercase item types A through Z have priorities 27 through 52.
        sum += match common_char {
            'a'..='z' => common_char as u32 - 'a' as u32 + 1,
            'A'..='Z' => common_char as u32 - 'A' as u32 + 27,
            _ => panic!("Invalid input"),
        };
    }
    println!("{}", sum);
}
