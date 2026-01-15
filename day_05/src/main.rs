use std::{
    cmp::max, fs::File, io::{BufReader, Lines}, usize
};

use shared::solve_puzzel_with_given_args;

#[derive(Debug, Clone, Copy)]
struct IdRange {
    start: usize,
    end: usize,
}

enum ParsingState {
    RangeParsing,
    IdParsing,
}

impl IdRange {
    fn new(start: usize, end: usize) -> IdRange {
        if start > end {
            dbg!(start, end);
            panic!("end needs to be greater or equal than start");
        }
        IdRange { start, end }
    }

    fn parse_from_input(input_line: String) -> IdRange {
        let sub_strings = input_line.split("-").collect::<Vec<&str>>();
        if sub_strings.iter().count() != 2 {
            panic!("Malformed input")
        }
        let start = sub_strings
            .first()
            .expect("sub_strings should be two so it has a first element")
            .parse::<usize>()
            .expect("start Input should be parsable to usize");
        let end = sub_strings
            .last()
            .expect("sub_strings should be two so it has a last element")
            .parse::<usize>()
            .expect("end Input should be parsable to usize");

        IdRange::new(start, end)
    }

    fn get_length(&self) -> usize {
        self.end - self.start + 1
    }
}

fn parse_input(input_lines: Lines<BufReader<File>>) -> (Vec<IdRange>, Vec<usize>) {
    let mut current_parsing_state = ParsingState::RangeParsing;

    let mut unsorted_overlapping_ranges: Vec<IdRange> = Vec::new();
    let mut ids_to_check: Vec<usize> = Vec::new();
    for line_result in input_lines {
        let Ok(line) = line_result else {
            break;
        };

        if line.is_empty() {
            current_parsing_state = ParsingState::IdParsing;

            continue;
        }
        match current_parsing_state {
            ParsingState::RangeParsing => {
                let range = IdRange::parse_from_input(line);
                unsorted_overlapping_ranges.push(range);
            }
            ParsingState::IdParsing => {
                let id = line
                    .parse::<usize>()
                    .expect("The id should be parsable to usize");
                ids_to_check.push(id);
            }
        }
    }
    (unsorted_overlapping_ranges, ids_to_check)
}

fn merge_ranges_and_check_if_done(sorted_overlapping_ranges: Vec<IdRange>) -> (Vec<IdRange>, bool) {
    let mut sorted_overlapping_ranges = sorted_overlapping_ranges.iter().peekable();

    let mut less_overlapping_ranges: Vec<IdRange> = Vec::new();
    let mut is_done_merging = true;
    while let Some(range) = sorted_overlapping_ranges.next() {
        let Some(next_range) = sorted_overlapping_ranges.peek() else {
            less_overlapping_ranges.push(*range);
            break;
        };

        if next_range.start > range.end {
            // Ranges do not overlap
            less_overlapping_ranges.push(*range);
            continue;
        }
        // If we merge, we can't be sure if we need to merge the new range in the next iteration
        is_done_merging = false;
        // Ranges overlap
        let max_end = max(range.end, next_range.end);
        let new_combined_range = IdRange::new(range.start, max_end);
        less_overlapping_ranges.push(new_combined_range);
        /*
        println!(
            "{range:?} and {next_range:?} are overlapping and are merged into {new_combined_range:?}"
        );
         */
        // Since we merged the current range with the next range, we need to consume the next range in order
        sorted_overlapping_ranges.next();
    }
    (less_overlapping_ranges, is_done_merging)
}

fn is_food_with_id_fresh(food_id: usize, fresh_id_ranges: &Vec<IdRange>) -> bool {
    for id_range in fresh_id_ranges {
        if food_id < id_range.start {
            // The fresh id_ranges are sorted so if the id that we test is lower than the start of the current range,
            // we can safely assume that there wont be any range after this range in which the food_id could be included,
            // so we can safley return false
            //return false;
            continue;
        }
        if food_id > id_range.end {
            continue;
        }

        return true;
    }
    return false;
}

fn solve_part_one(input_lines: Lines<BufReader<File>>) {
    let (mut unsorted_overlapping_ranges, ids_to_check) = parse_input(input_lines);
    unsorted_overlapping_ranges.sort_by_key(|range: &IdRange| range.start);
    let mut sorted_ranges = unsorted_overlapping_ranges;
    let mut is_done_merging = false;
    while !is_done_merging {
        (sorted_ranges, is_done_merging) = merge_ranges_and_check_if_done(sorted_ranges);
    }
    let mut fresh_food_counter = 0;
    for food_id in ids_to_check {
        println!("checking {food_id}");
        if is_food_with_id_fresh(food_id, &sorted_ranges) {
            fresh_food_counter += 1;
        }
    }
    println!("There are {fresh_food_counter} fresh ingredients availiable.");
}

fn solve_part_two(input_lines: Lines<BufReader<File>>) {
    let (mut unsorted_overlapping_ranges, _ids_to_check) = parse_input(input_lines);
    unsorted_overlapping_ranges.sort_by_key(|range: &IdRange| range.start);
    let mut sorted_ranges = unsorted_overlapping_ranges;
    let mut is_done_merging = false;
    while !is_done_merging {
        (sorted_ranges, is_done_merging) = merge_ranges_and_check_if_done(sorted_ranges);
    }
    let mut fresh_ingredient_id_count = 0;
    for id_range in sorted_ranges {
        fresh_ingredient_id_count += id_range.get_length();
    }
    println!("There are {fresh_ingredient_id_count} Ids considered fresh.");
}

fn main() {
    solve_puzzel_with_given_args("5", solve_part_one, solve_part_two);
}
