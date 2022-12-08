struct TreeGrid {
    height: usize,
    width: usize,
    grid: Vec<Vec<u32>>, // int is the height of the tree
}

impl TreeGrid {
    fn from_vector(grid: Vec<Vec<u32>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        TreeGrid {
            height,
            width,
            grid,
        }
    }

    fn count_visible_trees(&self) -> u32 {
        // get the top, left, bottom, right edge of the grid

        let mut count: u32 = 0;

        // then we consider the interior of the grid
        for i in 0..self.height {
            for j in 0..self.width {
                let is_visible = self.is_visible_from(Direction::Left, (i, j))
                    || self.is_visible_from(Direction::Right, (i, j))
                    || self.is_visible_from(Direction::Top, (i, j))
                    || self.is_visible_from(Direction::Bottom, (i, j));
                if is_visible {
                    count += 1;
                }
            }
        }
        count
    }

    fn view_distance(&self, dir: Direction, tree: (usize, usize)) -> u32 {
        match dir {
            Direction::Left => {
                if tree.1 == 0 {
                    return 0;
                }
                // check that every tree in the row is lower than the tree
                for i in (0..tree.1).rev() {
                    if self.grid[tree.0][i] >= self.grid[tree.0][tree.1] {
                        return (tree.1 - i) as u32;
                    }
                }
                tree.1 as u32
            }
            Direction::Right => {
                if tree.1 == self.width - 1 {
                    return 0;
                }
                // check that every tree in the row is lower than the tree
                for i in tree.1 + 1..self.width {
                    if self.grid[tree.0][i] >= self.grid[tree.0][tree.1] {
                        return (i - tree.1) as u32;
                    }
                }
                (self.width - 1 - tree.1) as u32
            }
            Direction::Top => {
                if tree.0 == 0 {
                    return 0;
                }
                // check that every tree in the column is lower than the tree
                for i in (0..tree.0).rev() {
                    if self.grid[i][tree.1] >= self.grid[tree.0][tree.1] {
                        return (tree.0 - i) as u32;
                    }
                }
                tree.0 as u32
            }
            Direction::Bottom => {
                if tree.0 == self.height - 1 {
                    return 0;
                }
                // check that every tree in the column is lower than the tree
                for i in tree.0 + 1..self.height {
                    if self.grid[i][tree.1] >= self.grid[tree.0][tree.1] {
                        return (i - tree.0) as u32;
                    }
                }
                (self.height - 1 - tree.0) as u32
            }
        }
    }

    fn is_visible_from(&self, dir: Direction, tree: (usize, usize)) -> bool {
        match dir {
            Direction::Left => {
                if tree.1 == 0 {
                    return true;
                }
                // check that every tree in the row to the left is lower than the tree
                for i in 0..tree.1 {
                    if self.grid[tree.0][i] >= self.grid[tree.0][tree.1] {
                        return false;
                    }
                }
                true
            }
            Direction::Right => {
                if tree.1 == self.width - 1 {
                    return true;
                }
                // check that every tree in the row to the right is lower than the tree
                for i in tree.1 + 1..self.width {
                    if self.grid[tree.0][i] >= self.grid[tree.0][tree.1] {
                        return false;
                    }
                }
                true
            }
            Direction::Top => {
                if tree.0 == 0 {
                    return true;
                }
                // check that every tree in the column above is lower than the tree
                for i in 0..tree.0 {
                    if self.grid[i][tree.1] >= self.grid[tree.0][tree.1] {
                        return false;
                    }
                }
                true
            }
            Direction::Bottom => {
                if tree.0 == self.height - 1 {
                    return true;
                }
                // check that every tree in the column below is lower than the tree
                for i in tree.0 + 1..self.height {
                    if self.grid[i][tree.1] >= self.grid[tree.0][tree.1] {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn get_best_scenic_score(&self) -> u32 {
        let mut best_score = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                let score_left = self.view_distance(Direction::Left, (i, j));
                let score_right = self.view_distance(Direction::Right, (i, j));
                let score_top = self.view_distance(Direction::Top, (i, j));
                let score_bottom = self.view_distance(Direction::Bottom, (i, j));
                let score = score_left * score_right * score_top * score_bottom;
                if score > best_score {
                    best_score = score;
                }
            }
        }
        best_score
    }
}

enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

fn read_input(file: &str) -> TreeGrid {
    let input = std::fs::read_to_string(file).unwrap();
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    TreeGrid::from_vector(grid)
}

pub fn part1() {
    let grid = read_input("input/day08.in");
    let trees = grid.count_visible_trees();
    println!("Part 1: {}", trees);
}

pub fn part2() {
    let grid = read_input("input/day08.in");
    let score = grid.get_best_scenic_score();
    println!("Part 2: {}", score);
}
