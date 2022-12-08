fn main() {
    let contents = include_str!("input.txt");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn elf_calories_iter(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.split("\n\n").map(|elf_calories| {
        elf_calories
            .lines()
            .map(|colorie| colorie.parse::<u64>().unwrap())
            .sum()
    })
}

fn part1(input: &str) -> u64 {
    elf_calories_iter(input).max().unwrap()
}

fn part2(input: &str) -> u64 {
    let mut highest = [0, 0, 0];
    for calories in elf_calories_iter(input) {
        if calories > highest[0] {
            highest[0] = calories;
            highest.sort();
        }
    }
    highest[0] + highest[1] + highest[2]
}
