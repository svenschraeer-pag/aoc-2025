use clap::{Arg, ArgAction, Command, value_parser};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::{Path, PathBuf},
};

pub fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn solve_puzzel_with_given_args(
    day: &str,
    solve_part1: fn(Lines<BufReader<File>>),
    solve_part2: fn(Lines<BufReader<File>>),
) {
    let matches = Command::new("aocer")
        .version("0.1")
        .about("Help you run your solutions for the given day")
        .arg(
            Arg::new("part")
                .long("part")
                .short('p')
                .value_parser(value_parser!(u8).range(1..=2))
                .action(ArgAction::Set)
                .default_value("1"),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .action(ArgAction::Set)
                .default_value("input.txt")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let file_name = matches
        .get_one::<PathBuf>("file")
        .expect("file has a default value");

    let part = matches
        .get_one::<u8>("part")
        .expect("part has a default value");

    let day_string: String = if day.len() == 1 {
        let mut temp = String::from("0");
        temp.push_str(day);
        temp
    } else {
        String::from(day)
    };

    let mut path = PathBuf::from("./");
    path.push(format!("day_{}", day_string));
    path.push(file_name);

    let input_lines = read_lines(dbg!(path)).expect("The filepath does not point to a valid file.");

    match part {
        1 => solve_part1(input_lines),
        2 => solve_part2(input_lines),
        _ => panic!("Can't solve part because its out of range of valid parts"),
    }
}
