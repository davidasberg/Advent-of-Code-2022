const DECRYPTION_KEY: i64 = 811_589_153;

struct EncryptedFile {
    original_list: Vec<i64>,
    indices: Vec<usize>,
}

impl EncryptedFile {
    fn mix(&mut self) {
        for i in 0..self.original_list.len() {
            let idx = self.indices.iter().position(|&x| x == i).unwrap();
            self.indices.remove(idx);
            let num = self.original_list[i];
            let new_idx = (idx as i64 + num).rem_euclid(self.indices.len() as i64) as usize;
            self.indices.insert(new_idx, i);
        }
    }

    fn get_mixed_list(&self) -> Vec<i64> {
        self.indices
            .iter()
            .map(|&i| self.original_list[i])
            .collect()
    }
}

fn read_input(file: &str, decryption_key: Option<i64>) -> EncryptedFile {
    let input = std::fs::read_to_string(file).expect("Error reading input file!");
    let decryption_key = decryption_key.unwrap_or(1);
    let list: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * decryption_key)
        .collect();

    EncryptedFile {
        original_list: list.clone(),
        indices: (0..list.len()).collect(),
    }
}

pub fn part1() {
    let mut file = read_input("input/day20.in", None);
    file.mix();
    let mixed_list = file.get_mixed_list();
    let zero_idx = mixed_list.iter().position(|&x| x == 0).unwrap();
    // get value at 1000, 2000, 3000 after zero_idx
    let sum_grove_coordinates = [1000, 2000, 3000]
        .iter()
        .map(|i| {
            let idx = (zero_idx + i).rem_euclid(mixed_list.len());
            println!("{}: {}", i, mixed_list[idx]);
            mixed_list[idx]
        })
        .sum::<i64>();
    println!("Sum of grove coordinates: {}", sum_grove_coordinates);
}

pub fn part2() {
    let mut file = read_input("input/day20.in", Some(DECRYPTION_KEY));
    for _ in 1..=10 {
        file.mix();
    }

    let mixed_list = file.get_mixed_list();

    let zero_idx = mixed_list.iter().position(|&x| x == 0).unwrap();
    println!("Zero index: {}", zero_idx);
    // get value at 1000, 2000, 3000 after zero_idx
    let sum_grove_coordinates = [1000, 2000, 3000]
        .iter()
        .map(|i| {
            let idx = (zero_idx + i).rem_euclid(mixed_list.len());
            println!("{}: {}", i, mixed_list[idx]);
            mixed_list[idx] as i128
        })
        .sum::<i128>();

    println!("Sum of grove coordinates: {}", sum_grove_coordinates);
}
