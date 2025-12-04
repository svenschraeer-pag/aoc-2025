use shared::read_lines;
use std::path::Path;

fn parse_range(range_str: &str) -> (i64, i64) {
    let range = range_str
        .split("-")
        .map(|e| {
            e.parse::<i64>()
                .expect("Should be able to be parsed as i64")
        })
        .collect::<Vec<i64>>();
    let start = range.first().expect("Range should have a start").to_owned();
    let end = range.last().expect("Ranmge should have an end").to_owned();
    (start, end)
}

fn is_repeated_sequence(word: &str) -> bool {
    let length = word.len();
    if (length % 2) != 0 {
        return false;
    }
    let first_half = &word[..(length / 2)];
    let second_half = &word[(length / 2)..];

    let contains = first_half.contains(second_half);
    //println!("{first_half} = {second_half} {contains}");

    contains
}
#[allow(dead_code)]
fn solve_part_one(ranges: impl Iterator<Item = (i64, i64)>) {
    let mut sum_of_wrong_ids: i64 = 0;
    for range in ranges {
        dbg!(range.0);
        dbg!(range.1);
        let amount: i64 = (range.0..=range.1)
            .map(|i| i.to_string())
            .filter(|i| is_repeated_sequence(&i))
            .filter_map(|s| s.parse::<i64>().ok())
            .sum();
        println!("{amount}");
        sum_of_wrong_ids += amount;
    }
    println!("Final sum of wrong ids: {sum_of_wrong_ids}");
}

fn are_all_chunks_sequence(chunks: impl Iterator<Item = String>,  sequence: &String) -> bool {
    for chunk in chunks {
        if !chunk.contains(sequence) {
            return false;
        }
    }
    return true; 
}

fn is_multiple_repeated_sequence(word: &str) -> bool {
    let length: usize = word.len();

    for i in 1..=length / 2 {
        if (length % i) != 0 {
            continue;
        }
        let chars = word.chars().collect::<Vec<char>>();
        let mut chunks = chars
            .chunks(i)
            .map(|s| s.iter().collect::<String>());
        let Some(sequence) = chunks.next() else {
            continue;
        };
        if !are_all_chunks_sequence(chunks, &sequence) {
            continue;
        }
        println!("{word} is good");
        return true;
    }

    return false;
}

fn solve_part_two(ranges: impl Iterator<Item = (i64, i64)>) {
    let mut sum_of_wrong_ids: i64 = 0;
    for range in ranges {
        dbg!(range.0);
        dbg!(range.1);
        let amount: i64 = (range.0..=range.1)
            .map(|i| i.to_string())
            .filter(|i| is_multiple_repeated_sequence(&i))
            .filter_map(|s| s.parse::<i64>().ok())
            .inspect(|i| println!("{i}"))
            .sum();
        println!("Will be adding {amount}");
        sum_of_wrong_ids += amount;
    }
    println!("Final sum of wrong ids: {sum_of_wrong_ids}");
}

fn main() {
    let path: &Path = Path::new("./day_02/input.txt");
    let mut input_lines = read_lines(path).expect("input.txt needs to be added to /day_02/");
    let Some(Ok(input_string)) = input_lines.nth(0) else {
        println!("The input is malformed");
        return;
    };
    let range_inputs = input_string.split(",");
    let ranges = range_inputs.map(|range| parse_range(range));
    solve_part_two(ranges);
}
