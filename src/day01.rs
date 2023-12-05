use std::io;
use std::io::prelude::*;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let calibration_values = parse(input)?;

    let now = std::time::Instant::now();
    let solution = match part {
        0 => (part_1(&calibration_values), part_2(&calibration_values)),
        1 => (part_1(&calibration_values), None),
        2 => (None, part_2(&calibration_values)),
        _ => unimplemented!(),
    };
    let time = now.elapsed().as_micros();

    match solution.0 {
        Some(x) => println!("Part 1: {}", x),
        None => println!(),
    }
    match solution.1 {
        Some(x) => println!("Part 2: {}", x),
        None => println!(),
    }

    println!("Time elapsed: {} µs", time);

    Ok(())
}

fn parse(input: impl BufRead) -> io::Result<Vec<u8>> {
    let mut calibration_values: Vec<u8> = Vec::new();

    for line in input.lines() {
        let s: String = line?;

        let mut value_str: String = String::new();
        s.chars()
            .filter(|&x| x.is_digit(10))
            .for_each(|x| value_str.push(x));

        let calibration_value: u8 = value_str.parse::<u8>().unwrap();

        calibration_values.push(calibration_value);
    }

    Ok(calibration_values)
}

fn part_1(calibration_values: &Vec<u8>) -> Option<u8> {
    /* Find the most calories carried! */
    Some(calibration_values.iter().sum())
}

fn part_2(calibration_values: &Vec<u8>) -> Option<i32> {
    unimplemented!()
}
