use std::fs::read_to_string;

fn find_largest_battery(line: &String, start: usize, end: usize) -> (usize, char) {
    return line[start..end].char_indices().fold(
        (0 as usize, '0'),
        |(acc_idx, acc_c), (idx, c)| {
            if c > acc_c {
                (idx, c)
            } else {
                (acc_idx, acc_c)
            }
        },
    );
}

fn find_highest_joltage(line: &String, start: usize, n_batteries_left: usize, acc: i64) -> i64 {
    let (first_battery_index, first_battery_value) =
        find_largest_battery(&line, start, line.len() - n_batteries_left);

    let joltage = acc * 10 + (first_battery_value as i64 - '0' as i64);
    if n_batteries_left == 0 {
        return joltage;
    } else {
        return find_highest_joltage(
            line,
            start + first_battery_index + 1,
            n_batteries_left - 1,
            joltage,
        );
    }
}

fn main() {
    let mut result1: i64 = 0;
    let mut result2: i64 = 0;

    for line in read_lines("input.txt") {
        result1 += find_highest_joltage(&line, 0, 1, 0);
        result2 += find_highest_joltage(&line, 0, 11, 0);
    }

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
