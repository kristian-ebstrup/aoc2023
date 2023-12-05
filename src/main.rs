extern crate clap;

use clap::{App, Arg};
use std::io;

mod day01;

fn main() -> io::Result<()> {
    let args = App::new("Advent of Code 2023")
        .version("0.1.0")
        .author("K. Ebstrup <k.ebstrup@gmail.com>")
        .about("My solution code to the Advent of Code 2023")
        .arg(Arg::new("day").required(false))
        .arg(Arg::new("part").required(false))
        .get_matches();

    let day = match args.value_of("day") {
        Some(s) => match s.parse::<u8>() {
            Ok(d) => d,
            Err(_e) => panic!("Error parsing the day!"),
        },
        None => 0,
    };

    let part = match args.value_of("part") {
        Some(s) => match s.parse::<u8>() {
            Ok(p) => p,
            Err(_e) => panic!("Error parsing the part!"),
        },
        None => 0,
    };

    // run the day
    let mut days_to_run: Vec<u8> = match day {
        0 => (1..9).map(|i| i).collect::<Vec<u8>>(),
        _ => vec![day],
    };

    while !days_to_run.is_empty() {
        let day_to_run = days_to_run.pop().unwrap();

        // get input file for the day
        let input = aoc2023::input_file(day_to_run)?;

        println!("# ---- DAY {:0>2} ---- #", day_to_run);
        match day_to_run {
            1 => day01::solve(input, part)?,
            _ => unimplemented!(),
        }
        println!(" ");
    }

    Ok(())
}
