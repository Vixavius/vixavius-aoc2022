// ------------------------------------------------------------------------------------------------
// Advent of Code 2022 - Day 1 - Part 2
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2022/day/2
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 2, 2022
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> i128 {
    let mut elves = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day01.txt") {
        // Parse each line as integer from text file, force unwrapping Result
        let mut elf_calories: u32 = 0;

        for line in lines {
            if let Ok(ip) = line {
                if ip.len() > 0 {
                    elf_calories += ip.parse::<u32>().unwrap();
                } else {
                    elves.push(elf_calories);
                    elf_calories = 0;
                }
            }
        }
        elves.push(elf_calories);
    } else {
        println!("Failed to open file");
    }

    let mut max: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;

    for elf in elves {
        if elf > max {
            third = second;
            second = max;
            max = elf;
        } else if elf > second {
            third = second;
            second = elf;
        } else if elf > third {
            third = elf;
        }
    }

    return (max + second + third) as i128;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
