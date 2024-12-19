use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("No input file found.");

    let re_mul = Regex::new(r"(mul\(\d+,\d+\))").expect("Mul regexp pattern is invalid.");
    let re_num = Regex::new(r"(\d+)").expect("Number extract regex pattern invalid.");

    let mut parts_between_denies = input.split("don't()");

    // Thinking time.
    // Patterns look like this:
    // Until first don't: We can accept everything
    // Between 2 don'ts: We're ignoring everything until the first do. After the first do, we're taking everything

    let first = parts_between_denies.nth(0).unwrap_or("");
    let rest: Vec<_> = parts_between_denies
        .map(|word| {
            word.split("do()")
                .skip(1)
                .fold(String::new(), |acc, w| acc + w)
        })
        .collect();

    let operations_from_first = re_mul.find_iter(&first).map(|w| w.as_str());
    let operations_from_rest = rest
        .iter()
        .flat_map(|part| re_mul.find_iter(part.as_str()).map(|w| w.as_str()));

    let operations = operations_from_first.chain(operations_from_rest);
    let solution = operations
        .map(|op| {
            re_num
                .find_iter(op)
                .map(|w| w.as_str().parse::<u32>().unwrap_or(1))
                .fold(1, |acc, x| acc * x)
        })
        .sum::<u32>();

    println!("{}", solution);
}
