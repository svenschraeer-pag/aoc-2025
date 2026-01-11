use shared::solve_puzzel_with_given_args;
use std::{
    fs::File,
    io::{BufReader, Lines},
};

fn get_highest_joltage(line: String) -> u32 {
    let mut numbers_after_highest: Vec<u32> = Vec::new();
    let mut chars_as_numbers = line
        .chars()
        .map(|char| char.to_digit(10))
        .flatten()
        .peekable();

    let mut highest = 0;
    while let Some(number) = chars_as_numbers.next() {
        if chars_as_numbers.peek().is_none() {
            numbers_after_highest.push(number);
            break;
        }
        if number <= highest {
            numbers_after_highest.push(number);
            continue;
        }
        numbers_after_highest.clear();
        highest = number;
    }
    let Some(second_highest) = numbers_after_highest.iter().max() else {
        panic!("there should be atleast 1 number after the highest");
    };
    highest * 10 + second_highest
}

// 1. Find the highest number each line without the last number
// 2. Find the highest number after the first highest number

fn solve_part_one(input_lines: Lines<BufReader<File>>) {
    let mut overall_joltage: Vec<u32> = Vec::new();
    for line_result in input_lines {
        let Ok(line) = line_result else {
            return;
        };
        overall_joltage.push(get_highest_joltage(line));
    }
    let total_output_joltage: u32 = overall_joltage.into_iter().sum();
    println!("Total joltage output: {total_output_joltage}")
}

// Part 2

// 1. Find the highest digit from the line without the last 11 digits
// 2. Take all remaining (appearing after the highest digit) digits
// and push the first digit of the prevoius saved 11 digits to the rest
// (resulting in a 10 saved digits)
// 2.1 If there is no rest then push the previous saved numbers as total
// repeat so on

fn get_highest_digit_with_rest(searchable_digits: Vec<u32>) -> (u32, Vec<u32>) {
    let mut numbers_after_highest: Vec<u32> = Vec::new();
    let mut highest_digit = 0;
    for digit in searchable_digits {
        if digit <= highest_digit {
            numbers_after_highest.push(digit);
            continue;
        }
        numbers_after_highest.clear();
        highest_digit = digit;
    }
    (highest_digit, numbers_after_highest)
}

fn calculate_joltage_of_line(highest_digits_in_order_of_appearance: Vec<u32>) -> u64 {
    let number_string = highest_digits_in_order_of_appearance
        .into_iter()
        .map(|digit| char::from_digit(digit, 10).expect("all digits must be valid for base 10"))
        .collect::<String>();
    number_string
        .parse::<u64>()
        .expect("Number string should fit to u64")
}

fn get_highest_joltage_from_line_with_battery_count(line: String, battery_count: usize) -> u64 {
    let mut chars_as_numbers = line.chars().map(|char| {
        char.to_digit(10)
            .expect("The given char should be able to be converted to a digit in base10")
    });
    // example 12 batteries Remove the last 11 digits
    let mut last_digits = chars_as_numbers
        .by_ref()
        .rev()
        .take(battery_count)
        .collect::<Vec<u32>>();

    let mut searchable_digits = chars_as_numbers.collect::<Vec<u32>>();

    let mut found_highest_digits: Vec<u32> = Vec::new();

    for _current_count in (1..=battery_count).rev() {
        let last_digit = last_digits
            .pop()
            .expect("Last digits should have enough values");
        searchable_digits.push(last_digit);
        let highest_digit;
        (highest_digit, searchable_digits) = get_highest_digit_with_rest(searchable_digits);
        found_highest_digits.push(highest_digit);
        // Early found when there is no rest then the last saved digits are sufficient
        if searchable_digits.is_empty() {
            last_digits.reverse();
            found_highest_digits.append(&mut last_digits);
            dbg!("Early found");
            break;
        }
    }

    let total_joltage = calculate_joltage_of_line(found_highest_digits);
    dbg!(total_joltage)
}

fn solve_part_two(input_lines: Lines<BufReader<File>>) {
    let mut overall_joltage: Vec<u64> = Vec::new();
    for line_result in input_lines {
        let Ok(line) = line_result else {
            return;
        };
        let line_joltage = get_highest_joltage_from_line_with_battery_count(line, 12);
        overall_joltage.push(line_joltage);
    }

    let total_output_joltage: u64 = overall_joltage.into_iter().sum();
    println!("Total joltage output: {total_output_joltage}")
}

fn main() {
    solve_puzzel_with_given_args("3", solve_part_one, solve_part_two);
}
