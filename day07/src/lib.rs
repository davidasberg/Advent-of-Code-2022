use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq)]
struct Directory {
    parent: Option<Rc<RefCell<Directory>>>,
    children: HashMap<String, Rc<RefCell<Directory>>>,
    files: HashMap<String, usize>,
    size: Option<usize>,
}

impl std::fmt::Debug for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Directory")
            .field("parent", &self.parent)
            .field("children", &self.children.len())
            .field("files", &self.files)
            .field("size", &self.size)
            .finish()
    }
}

impl Directory {
    fn new() -> Self {
        Directory {
            parent: None,
            children: HashMap::new(),
            files: HashMap::new(),
            size: None,
        }
    }

    fn size(&mut self) -> usize {
        self.calc_sizes();
        self.size.unwrap()
    }

    fn calc_sizes(&mut self) {
        let mut size = 0;
        for file_size in self.files.values() {
            size += file_size;
        }
        for child in self.children.values() {
            child.borrow_mut().calc_sizes();
            size += child.borrow().size.unwrap();
        }
        self.size = Some(size);
    }

    fn directories_size(&mut self) -> Vec<usize> {
        self.calc_sizes();
        let mut dirs = Vec::new();

        dirs.push(self.size.unwrap());

        for child in self.children.values() {
            dirs.extend(child.borrow_mut().directories_size());
        }

        dirs
    }
}

fn read_input(file: &str) -> Rc<RefCell<Directory>> {
    // commands start with $
    let input = std::fs::read_to_string(file).unwrap();
    let mut lines = input.lines();
    let root = Rc::new(RefCell::new(Directory::new()));
    let mut current_directory = Rc::clone(&root);

    while let Some(line) = lines.next() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        match tokens[..] {
            ["$", "cd", dir] => {
                current_directory = match dir {
                    ".." => {
                        if let Some(parent) = &current_directory.borrow().parent {
                            Rc::clone(parent)
                        } else {
                            println!("Cannot cd .. from root");
                            current_directory.clone()
                        }
                    }
                    "/" => root.clone(),
                    dir => {
                        insert_child(Rc::clone(&current_directory), dir);

                        current_directory
                            .borrow_mut()
                            .children
                            .get(dir)
                            .unwrap()
                            .clone()
                    }
                };
            }
            ["$", "ls"] => {}
            ["dir", name] => {
                insert_child(Rc::clone(&current_directory), name);
            }
            [size, name] => {
                let size = size.parse::<usize>().unwrap();
                current_directory
                    .borrow_mut()
                    .files
                    .insert(name.to_string(), size);
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
    root
}

fn insert_child(current_directory: Rc<RefCell<Directory>>, name: &str) {
    let mut dir = Directory::new();
    dir.parent = Some(Rc::clone(&current_directory));

    current_directory
        .borrow_mut()
        .children
        .insert(name.to_string(), Rc::new(RefCell::new(dir)));
}

pub fn part1() {
    let root = read_input("input/day07.in");

    let sum: usize = root
        .borrow_mut()
        .directories_size()
        .iter()
        .filter(|x| **x < 100000)
        .sum();

    println!("Sum: {}", sum);
}

pub fn part2() {
    let total_disk_space = 70000000;
    let space_needed = 30000000;

    let root = read_input("input/day07.in");
    let used = root.borrow_mut().size();
    let need_to_free = space_needed - (total_disk_space - used);
    let dirs = root.borrow_mut().directories_size();
    let min = dirs.iter().filter(|x| **x > need_to_free).min().unwrap();
    println!("Min: {}", min);
}
