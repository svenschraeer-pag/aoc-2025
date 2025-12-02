use shared::read_lines;
use std::{
    char,
    fs::File,
    io::{BufReader, ErrorKind, Lines},
    path::{Path},
};

fn get_diretion_multiplier(instruction_char: &char) -> Result<i32, std::io::ErrorKind> {
    match instruction_char {
        'L' => Ok(-1),
        'R' => Ok(1),
        _ => Err(ErrorKind::InvalidData),
    }
}

fn parse_input_to_distance(input: &str) -> Result<i32, ErrorKind> {
    let Some(first_char) = input.chars().nth(0) else {
        return Err(ErrorKind::InvalidData);
    };
    let direction = get_diretion_multiplier(&first_char)?;
    let Ok(distance) = input[1..].parse::<i32>() else {
        return Err(ErrorKind::InvalidData);
    };
    Ok(direction * distance)
}

#[allow(dead_code)]
fn solve_part_one(input_lines: Lines<BufReader<File>>) {
    let (count, _last_sum) = input_lines
        .filter_map(|line| line.ok())
        .filter_map(|input| parse_input_to_distance(&input).ok())
        .fold(
            (0, 50),
            |(mut relevant_turns, mut current_dial_pos), dial_turn| {
                current_dial_pos = (current_dial_pos + dial_turn) % 100;
                if current_dial_pos == 0 {
                    relevant_turns += 1
                }
                (relevant_turns, current_dial_pos)
            },
        );
    println!("The dial ended {count}-times in 0 position");
}

fn move_dial(dial_pos: &mut i32, distance: i32) -> i32 {
    let mut count = 0;
    //println!("{dial_pos} needs to be moved by {distance}");

    let increment = if distance > 0 { 1 } else { -1 };
    let abs_distance = distance.abs();

    for _ in 0..abs_distance {
        *dial_pos += increment;

        *dial_pos %= 100;

        if *dial_pos == 0 {
            count += 1;
        }
    }
    //println!("We increase by {count}");
    count
}

fn solve_part_two(input_lines: Lines<BufReader<File>>) {
    let mut dbg_count_before = 0;
    let mut dbg_count_after = 0;
    let input = input_lines
        .inspect(|_| dbg_count_before += 1)
        .filter_map(|line| line.ok())
        .filter_map(|input| parse_input_to_distance(&input).ok())
        .inspect(|_| dbg_count_after += 1);

    let mut current_dial_pos = 50;
    let mut count = 0;
    for dial in input {
        count += move_dial(&mut current_dial_pos, dial);
    }

    println!("The dial skipped {count}-times in 0 position");
}

fn main() {
    let path = Path::new("./day_01/input.txt");
    dbg!(path.display());
    let input_lines = read_lines(path).expect("input.txt should be included");
    //solve_part_one(input_lines);

    solve_part_two(input_lines);
}
