use std::{collections::HashMap, fs};

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

    let sum_part_1: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("{}", sum_part_1);

    let mut elements_with_count: HashMap<u32, u32> = HashMap::new();

    for element in &right {
        let count: u32 = u32::try_from(right.iter().filter(|x| *x == element).count())
            .expect("Count is way too big");

        elements_with_count.insert(*element, count);
    }

    let sum_part_2: u32 = left
        .iter()
        .map(|x| x * elements_with_count.get(x).unwrap_or(&0))
        .sum();

    println!("{}", sum_part_2);
}
