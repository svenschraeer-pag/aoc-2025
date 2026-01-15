# My Advent of Code Solutions 2025

**Disclaimer**: These are by no means the fastest, cleanest, most idiomatic or in any way perfect solutions to the given Puzzels of [Advent of Code 2025](https://adventofcode.com/2025/). 

I just want to expand my Rust skillset and learn to use the language by solving puzzels.

## How to (in case I forget how to use my commands)

```
cargo run -p day_{daynumber based on the folder name of the package} -- -p {part_number(1/2)} -f {file_name.txt (for puzzel input)}
```

Example:
`cargo run -p day_04 -- -p 2 -f test.txt`

## Basic Setup for new solutions
```rust
use std::{fs::File, io::{BufReader, Lines}};

use shared::solve_puzzel_with_given_args;


fn solve_part_one(input_lines: Lines<BufReader<File>>) {

}

fn solve_part_two(input_lines: Lines<BufReader<File>>) {
    
}

fn main() {
    solve_puzzel_with_given_args("5", solve_part_one, solve_part_two);
}
```