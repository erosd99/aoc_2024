use std::fs;

fn main() {
    let input = fs::read_to_string("data/input.txt").expect("Input file was not found!");

    let (left_with_index, right_with_index): (Vec<_>, Vec<_>) = input
        .lines()
        .flat_map(|x| x.split_whitespace())
        .filter_map(|x| x.parse::<u32>().ok())
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    let mut left: Vec<u32> = left_with_index.into_iter().map(|(_, val)| val).collect();
    let mut right: Vec<u32> = right_with_index.into_iter().map(|(_, val)| val).collect();
    left.sort();
    right.sort();

    let sum: u32 = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    println!("{}", sum);
}
