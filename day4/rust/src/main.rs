use std::fs::read_to_string;

fn count_neighbouring_rolls(
    grid: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    bounds: (i32, i32, i32, i32),
) -> i32 {
    let mut count = 0;

    let (x1, y1, x2, y2) = bounds;

    let neighbour_cells = [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ];

    for (cx, cy) in neighbour_cells {
        // Out of bounds
        if cx < x1 || cx > x2 || cy < y1 || cy > y2 {
            continue;
        }

        if grid[cy as usize][cx as usize] == '@' {
            count += 1;
        }
    }

    count
}

fn main() {
    let mut grid = read_lines("input.txt")
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut result1 = 0;
    let bounds = (0, 0, grid[0].len() as i32 - 1, grid.len() as i32 - 1);
    let mut result2 = 0;
    let mut n_iterations = 0;
    loop {
        let mut rolls_to_remove: Vec<(usize, usize)> = Vec::new();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] != '@' {
                    continue;
                }
                if count_neighbouring_rolls(&grid, x as i32, y as i32, bounds) < 4 {
                    if n_iterations == 0 {
                        result1 += 1;
                    }
                    result2 += 1;
                    rolls_to_remove.push((x, y));
                }
            }
        }

        if rolls_to_remove.len() == 0 {
            break;
        }

        for (x, y) in rolls_to_remove {
            grid[y][x] = '.';
        }
        n_iterations += 1;
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
