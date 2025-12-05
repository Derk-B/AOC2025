use std::cmp::max;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut ranges = Vec::<(i64, i64)>::new();

    let lines = read_lines("input.txt");

    let mut db_sections = lines.split(|s| s == "");

    // First part contains the 'fresh' ranges
    if let Some(section) = db_sections.next() {
        for line in section {
            let mut range_part_iterator = line.split('-');
            let begin = range_part_iterator
                .next()
                .unwrap_or_else(|| panic!("Failed to parse begin of range in line: {}", line))
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("Failed to parse begin of range in line: {}", line));

            let end = range_part_iterator
                .next()
                .unwrap_or_else(|| panic!("Failed to parse end of range in line: {}", line))
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("Failed to parse end of range in line: {}", line));

            ranges.push((begin, end));
        }
    }

    ranges.sort();
    let mut ranges2 = ranges.iter().peekable();
    let mut ranges_simplified = Vec::<(i64, i64)>::new();

    let mut new_range = ranges2.next().unwrap().clone();
    loop {
        if let Some(next) = ranges2.next() {
            let (b, e) = next.clone();
            if b <= new_range.1 {
                new_range.1 = max(e, new_range.1)
            } else {
                ranges_simplified.push(new_range);
                new_range = (b, e);
            }
        } else {
            ranges_simplified.push(new_range);
            break;
        }
    }

    let start2 = Instant::now();
    let mut result1: i32 = 0;
    if let Some(section) = db_sections.next() {
        for line in section {
            let ingredient_id = line
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("Failed to parse ingredient ID: {}", line));

            if ranges_simplified
                .iter()
                .any(|(b, e)| ingredient_id >= *b && ingredient_id <= *e)
            {
                result1 += 1;
            }
        }
    }

    let mut result2 = 0 as i64;
    for (b, e) in ranges_simplified {
        result2 += e - b + 1;
    }

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
    println!("Range check duration: {:?}", start2.elapsed());
    println!("Total duration: {:?}", start.elapsed());
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
