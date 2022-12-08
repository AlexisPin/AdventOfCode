use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

const MAX_SIZE: i32 = 100000;
const TOTAL_SPACE: i32 = 70000000;
const REQUIRED_FREE_SPACE: i32 = 30000000;

#[derive(Debug)]
enum ParseResult {
    CdRoot,
    CdUp,
    Cd(String),
    List,
    Dir(String),
    File((String, i32)),
}

fn main() {
    println!("Part 1: {}", solve01());
    println!("Part 2: {}", solve02());
}

pub fn solve01() -> i32 {
    use ParseResult::*;

    let mut sum = 0;
    let mut dir_stack: Vec<i32> = Vec::new();

    for pr in INPUT.lines().map(parse) {
        // dbg!(&pr, dir_sizes.len());
        match pr {
            CdRoot | Cd(_) => dir_stack.push(0),
            CdUp => {
                let size = dir_stack.pop().unwrap();
                increase_last(&mut dir_stack, size);
                if size <= MAX_SIZE {
                    sum += size;
                }
            }
            File((_, size)) => increase_last(&mut dir_stack, size),
            List | Dir(_) => {}
        }
    }

    while let Some(size) = dir_stack.pop() {
        if size <= MAX_SIZE {
            sum += size;
        }

        increase_last(&mut dir_stack, size);
    }

    sum
}

pub fn solve02() -> i32 {
    use ParseResult::*;

    let mut dir_stack: Vec<i32> = Vec::new();
    let mut dir_sizes: Vec<i32> = Vec::new();
    let mut sum = 0;
    for pr in INPUT.lines().map(parse) {
        match pr {
            CdRoot | Cd(_) => dir_stack.push(0),
            CdUp => {
                let size = dir_stack.pop().unwrap();
                dir_sizes.push(size);
                increase_last(&mut dir_stack, size);
            }
            File((_, size)) => {
                sum += size;
                increase_last(&mut dir_stack, size)
            }
            List | Dir(_) => {}
        }
    }

    while let Some(size) = dir_stack.pop() {
        dir_sizes.push(size);
        increase_last(&mut dir_stack, size);
    }

    let min_dir_size = sum + REQUIRED_FREE_SPACE - TOTAL_SPACE;
    dir_sizes
        .into_iter()
        .filter(|&size| size >= min_dir_size)
        .min()
        .unwrap()
}

fn increase_last(s: &mut [i32], size: i32) {
    let len = s.len();
    if len > 0 {
        s[len - 1] += size;
    }
}

fn parse(s: &str) -> ParseResult {
    use ParseResult::*;

    match s.split_once(' ').unwrap() {
        ("$", "ls") => List,
        ("$", command) => match command.split_once(' ').unwrap() {
            ("cd", "/") => CdRoot,
            ("cd", "..") => CdUp,
            ("cd", dir_name) => Cd(dir_name.to_owned()),
            _ => panic!("unknown command"),
        },
        ("dir", dir_name) => Dir(dir_name.to_owned()),
        (file_size, file_name) => File((file_name.to_owned(), i32::from_str(file_size).unwrap())),
    }
}
