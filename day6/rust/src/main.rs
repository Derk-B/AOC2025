use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input.txt");

    let numbers_grid = &lines[0..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.parse::<i64>()
                        .unwrap_or_else(|e| panic!("Failed to parse: '{}' to i64\n{:?}", s, e))
                })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    // Rotate
    let mut i = 0;
    let mut rotated_numbers_grid: Vec<Vec<i64>> = Vec::new();
    while i < numbers_grid[0].len() {
        let mut vertial_vec: Vec<i64> = Vec::new();
        for line in numbers_grid {
            vertial_vec.push(line[i]);
        }
        rotated_numbers_grid.push(vertial_vec);
        i += 1;
    }

    let operators = &lines[lines.len() - 1]
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut result1: i64 = 0;
    let mut i = 0;
    while i < rotated_numbers_grid.len() {
        if operators[i] == "+" {
            result1 += rotated_numbers_grid[i].iter().fold(0, |acc, n| acc + n);
        } else if operators[i] == "*" {
            result1 += rotated_numbers_grid[i].iter().fold(1, |acc, n| acc * n);
        }

        i += 1;
    }

    println!("Part 1: {}", result1);

    // Rotate raw input
    let mut rotated_number_input: Vec<Vec<char>> = Vec::new();
    let mut i = 0;
    while i < lines[0].len() {
        let mut vertical_str = Vec::<char>::new();
        for line in &lines[0..lines.len() - 1]
            .iter()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>()
        {
            vertical_str.push(line[i]);
        }

        rotated_number_input.push(vertical_str);
        i += 1;
    }

    let mut number_groups = Vec::<Vec<i64>>::new();
    let mut number_group = Vec::<i64>::new();
    for char_array in &rotated_number_input {
        let number_str = char_array
            .iter()
            .fold(String::new(), |acc, c| acc + &c.to_string());

        // println!("Str: {}, is empty: {}", number_str, number_str.is);
        if number_str.chars().all(|c| c == ' ') {
            number_groups.push(number_group.clone());
            number_group = Vec::new();
        } else {
            number_group.push(
                number_str
                    .trim()
                    .parse::<i64>()
                    .unwrap_or_else(|e| panic!("Failed to parse: {}, error: {:?}", number_str, e)),
            );
        }
    }
    number_groups.push(number_group);

    let mut result2: i64 = 0;
    let mut i = 0;
    while i < rotated_numbers_grid.len() {
        if operators[i] == "+" {
            result2 += number_groups[i].iter().fold(0, |acc, n| acc + n);
        } else if operators[i] == "*" {
            result2 += number_groups[i].iter().fold(1, |acc, n| acc * n);
        }

        i += 1;
    }

    println!("Part 2: {}", result2);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
