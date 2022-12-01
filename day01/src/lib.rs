use std::fs;

pub fn part1() {
    // read input
    //split input into vector of strings on empty lines

    let input = fs::read_to_string("input/day01.in").expect("Error reading file");
    let input_vec: Vec<&str> = input.split("\n\n").collect();

    let mut max = 0;
    for group in input_vec {
        //read every line in group and sum
        let mut sum = 0;
        let group_vec: Vec<&str> = group.split('\n').collect();
        //cast to int and sum
        for line in group_vec {
            sum += line.parse::<i32>().unwrap();
        }
        if sum > max {
            max = sum;
        }
    }

    println!("Max: {}", max);
}

pub fn part2() {
    // read input
    //split input into vector of strings on empty lines

    let input = fs::read_to_string("input/Calories.txt").expect("Error reading file");
    let input_vec: Vec<&str> = input.split("\n\n").collect();

    //top 3 max
    let mut sums: Vec<i32> = Vec::new();
    for group in input_vec {
        //read every line in group and sum
        let mut sum = 0;
        let group_vec: Vec<&str> = group.split('\n').collect();
        //cast to int and sum
        for line in group_vec {
            sum += line.parse::<i32>().unwrap();
        }

        //insert into sums
        sums.push(sum);
    }

    //sort descending
    sums.sort_by(|a, b| b.cmp(a));

    println!("{:?}", sums);

    println!("Max: {}", sums[0] + sums[1] + sums[2]);
}
