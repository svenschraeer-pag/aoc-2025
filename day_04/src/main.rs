use std::{
    fs::File,
    io::{BufReader, Lines},
};

use grid::*;

use shared::solve_puzzel_with_given_args;

static DEFAULT_8_NEIGHBOR_OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_neighbors_position_from_offset(
    pos: &(usize, usize),
    offset: (i8, i8),
) -> Option<(usize, usize)> {
    let Some(row_pos) = pos.0.checked_add_signed(offset.0.into()) else {
        return None;
    };
    let Some(col_pos) = pos.1.checked_add_signed(offset.1.into()) else {
        return None;
    };
    Some((row_pos, col_pos))
}

fn get_neighbors_in_grid(
    grid: &Grid<bool>,
    pos: (usize, usize),
    neighbor_offset_mask: Vec<(i8, i8)>,
) -> u8 {
    let mut neighbor_count = 0;

    for offset in neighbor_offset_mask {
        let Some(neighbor_pos) = get_neighbors_position_from_offset(&pos, offset) else {
            continue;
        };
        let Some(neighbor_value) = grid.get(neighbor_pos.0, neighbor_pos.1) else {
            // There is no roll or space on this positon
            // We could set some filters here to avoid over testing
            continue;
        };
        if *neighbor_value == false {
            // No roll on this spot
            continue;
        }
        neighbor_count += 1;
    }
    neighbor_count
}

fn fill_input_to_grid(input_lines: Lines<BufReader<File>>) -> Grid<bool> {
    let mut grid: Grid<bool> = grid![];
    for line_result in input_lines {
        let Ok(line) = line_result else {
            break;
        };
        let line_to_fill = line
            .chars()
            .filter_map(|char| match char {
                '@' => Some(true),
                '.' => Some(false),
                _ => None,
            })
            .collect::<Vec<bool>>();
        grid.push_row(line_to_fill);
    }
    grid
}

fn solve_part_one(input_lines: Lines<BufReader<File>>) {
    let grid = fill_input_to_grid(input_lines);

    let mut accessable_rolls_count: u32 = 0;

    let cols_amount = grid.cols();
    for current_row in 0..grid.rows() {
        for current_col in 0..cols_amount {
            let current_item: &bool = grid
                .get(current_row, current_col)
                .expect("Grid should return the item in the bounds");
            if *current_item == false {
                // No roll in this spot -> Don't need to check their neighbors
                continue;
            }
            let neighbor_count = get_neighbors_in_grid(
                &grid,
                (current_row, current_col),
                DEFAULT_8_NEIGHBOR_OFFSETS.to_vec(),
            );
            if neighbor_count >= 4 {
                continue;
            }
            accessable_rolls_count += 1;
        }
    }
    println!("{accessable_rolls_count} rolls are accessable by a forklift");
}

fn find_removable_rolls(
    grid: &Grid<bool>,
    accessable_rolls_count: &mut u32,
    removable_roll_positions: &mut Vec<(usize, usize)>,
) {
    let cols_amount = grid.cols();
    for current_row in 0..grid.rows() {
        for current_col in 0..cols_amount {
            let current_item: &bool = grid
                .get(current_row, current_col)
                .expect("Grid should return the item in the bounds");
            if *current_item == false {
                // No roll in this spot -> Don't need to check their neighbors
                continue;
            }
            let neighbor_count = get_neighbors_in_grid(
                &grid,
                (current_row, current_col),
                DEFAULT_8_NEIGHBOR_OFFSETS.to_vec(),
            );
            if neighbor_count >= 4 {
                continue;
            }
            removable_roll_positions.push((current_row, current_col));
            *accessable_rolls_count += 1;
        }
    }
}

fn remove_removable_rolls(grid: &mut Grid<bool>, removable_roll_positions: &mut Vec<(usize, usize)>) {
    for pos in &mut *removable_roll_positions {
        let roll = grid.get_mut(pos.0, pos.1).expect("The given position should be accessable in the grid");
        //To remove the roll set the value at the position to false
        *roll = false
    }
    let amount = removable_roll_positions.iter().count();
    println!("Now {amount} rolls are removed.");
    removable_roll_positions.clear();
}

fn solve_part_two(input_lines: Lines<BufReader<File>>) {
    let mut grid = fill_input_to_grid(input_lines);

    let mut accessable_rolls_count: u32 = 0;
    let mut removable_roll_positions: Vec<(usize, usize)> = Vec::new();

    loop {
        find_removable_rolls(&grid, &mut accessable_rolls_count, &mut removable_roll_positions);
        if removable_roll_positions.is_empty() {
            break;
        }
        remove_removable_rolls(&mut grid, &mut removable_roll_positions);
        println!("Currently {accessable_rolls_count} rolls were removed.")
    }
    println!("{accessable_rolls_count} rolls were removed in total.");
}

fn main() {
    solve_puzzel_with_given_args("4", solve_part_one, solve_part_two);
}
