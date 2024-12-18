use std::{cmp::Ordering, fs};
fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file not found!");

    let reports: Vec<Vec<_>> = input
        .lines()
        .filter_map(|line| {
            Some({
                line.split_whitespace()
                    .filter_map(|level| Some(level.parse::<u32>().unwrap_or_default())) // TODO: Why does this satisfy the compiler
                    .collect()
            })
        })
        .collect();

    let safe_reports: Vec<_> = reports
        .into_iter()
        // .skip(100)
        // .take(20)
        .filter(|report| fully_safe(report))
        .collect();
    println!("{}", safe_reports.len());
}

fn fully_safe(report: &[u32]) -> bool {
    let first_faulty_chunk = match get_idx_of_first_faulty_chunk(report) {
        Some(val) => val,
        None => return true, // AAAH WHAT
    };

    let without_first_char: Vec<_> = report
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| idx != &first_faulty_chunk)
        .map(|(_, val)| *val)
        .collect();

    let without_second_char: Vec<_> = report
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| idx != &(first_faulty_chunk + 1))
        .map(|(_, val)| *val)
        .collect();

    return match (
        get_idx_of_first_faulty_chunk(without_first_char.as_slice()),
        get_idx_of_first_faulty_chunk(without_second_char.as_slice()),
    ) {
        (None, _) => true,
        (_, None) => true,
        _ => false,
    };
}

fn get_idx_of_first_faulty_chunk(report: &[u32]) -> Option<usize> {
    let first_order = report[1].cmp(&report[0]);
    // Edge case: What if only the first order is wrong? Then we have to remove the first
    // Element to maybe make the solution correct.
    if report.len() >= 4 {
        match report.windows(4).next() {
            Some(special_chunk) => {
                let second_order = special_chunk[2].cmp(&special_chunk[1]);
                let third_order = special_chunk[3].cmp(&special_chunk[2]);

                match (first_order, second_order, third_order) {
                    (Ordering::Equal, _, _)
                    | (Ordering::Less, Ordering::Greater, Ordering::Greater)
                    | (Ordering::Greater, Ordering::Less, Ordering::Less) => return Some(0),
                    _ => (),
                }
            }
            None => (),
        }
    }

    for (idx, chunk) in report.windows(2).enumerate() {
        let current_order = chunk[1].cmp(&chunk[0]);
        let absolute_difference = chunk[1].abs_diff(chunk[0]);

        // TODO: WTF this is fugly
        let is_invalid_order = match (current_order, first_order) {
            (_, Ordering::Equal) => true,
            (Ordering::Equal, _) => true,
            (Ordering::Greater, Ordering::Less) => true,
            (Ordering::Less, Ordering::Greater) => true,
            _ => false,
        };
        let is_too_big_difference = absolute_difference > 3;

        if is_invalid_order || is_too_big_difference {
            // println!("{:?}", report);
            // println!("First order:{:?}", first_order);
            // println!("Current order:{:?}", current_order);
            // println!("Absolute difference: {}", absolute_difference);
            // println!("Index of faulty chunk: {}", idx);

            return Some(idx);
        }
    }

    return None;
}
