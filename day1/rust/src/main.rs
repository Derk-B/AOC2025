use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = read_lines("input.txt");

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    let mut dial: i32 = 50;
    for line in file.lines().map_while(Result::ok) {
        let prev: i32 = dial;
        if line.starts_with("L") {
            if let Ok(n) = line[1..].parse::<i32>() {
                dial -= n;
            }
        }
        if line.starts_with("R") {
            if let Ok(n) = line[1..].parse::<i32>() {
                dial += n;
            }
        }

        if dial % 100 == 0 {
            part1 += 1;
        }

        if dial <= 0 {
            if prev != 0 {
                part2 += dial.abs() / 100 + 1;
            } else {
                part2 += dial.abs() / 100;
            }
        } else if dial >= 100 {
            part2 += dial / 100;
        }
        dial = dial.rem_euclid(100);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn read_lines(filename: &str) -> io::BufReader<File> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => panic!("Error, could not open file with name: {}", filename),
    };
    let reader = io::BufReader::new(file);
    reader
}
