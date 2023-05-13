use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/input.prod").unwrap();

    first_part(&input);

    second_part(&input);
}

fn first_part(input: &str)
{
    let result: i32 = input.lines().map(|line| -> i32 {
        let chars = line.chars().collect::<Vec<char>>();
        let opponent_choice = chars[0];
        let choice = chars[2];

        match (opponent_choice, choice) {
            ('A', 'X') => 3 + 1,
            ('A', 'Y') => 6 + 2,
            ('A', 'Z') => 0 + 3,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 6 + 1,
            ('C', 'Y') => 0 + 2,
            ('C', 'Z') => 3 + 3,
            _ => -1,
        }
    }).sum();

    print!("First Problem: {}\n", result);
}

fn second_part(input: &str)
{
    let result: i32 = input.lines().map(|line| -> i32 {
        let chars = line.chars().collect::<Vec<char>>();
        let opponent_choice = chars[0];
        let choice = chars[2];

        match (opponent_choice, choice) {
            ('A', 'X') => 0 + 3,
            ('A', 'Y') => 3 + 1,
            ('A', 'Z') => 6 + 2,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 0 + 2,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 6 + 1,
            _ => -1,
        }
    }).sum();

    print!("Second Problem: {}\n", result);

}
