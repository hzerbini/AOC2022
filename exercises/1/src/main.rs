use std::fs::read_to_string;

fn main() {

    let input = read_to_string("input/prod.input").unwrap();

    let elves = parse_elves(input);

    part_one(&elves);

    part_two(&elves);
}

fn parse_elves(input: String) -> Vec<u32> {
    let mut elves: Vec<u32>  = input
        .split("\n\n")
        .map(|x: &str| -> u32 {
            x.split("\n")
                .map(|value| value.parse::<u32>().unwrap_or(0))
                .sum()
        })
        .collect();


    elves.sort_unstable();

    return elves;
}

fn part_one(elves: &Vec<u32>) {
    println!("Part One Result: {:?}", elves.last().unwrap());
}

fn part_two(elves: &Vec<u32>) {
    println!("Part Two Result: {:?}", elves.into_iter().rev().take(3).sum::<u32>());
}
