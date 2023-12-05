use std::fs;

fn main() {
    println!("Part 1");
    println!("{}", part1());
    println!("Part 2");
    println!("{}", part2());
}

fn get_puzzle_input() -> Vec<String> {
    let contents = fs::read_to_string("input")
        .expect("missing input file!");
    
    contents
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

struct NumberCandidate {
    row: usize,
    start: usize,
    end: usize,
    number: i32
}

fn get_row_numbers(row: &Vec<char>, row_index: usize) -> Vec<NumberCandidate> {
    let mut numbers: Vec<NumberCandidate> = vec!();
    let mut current_chars: Vec<char> = vec!();
    for (index, char) in row.iter().enumerate() {
        match (char, current_chars.len()) {
            ('0'..='9', _) => {
                current_chars.push(*char)
            }
            (_, 1..) => {
                let str: String = current_chars.into_iter().collect();
                current_chars = vec!();
                let num: i32 = str.parse().expect("failed to convert number");
                numbers.push(NumberCandidate { 
                    row: row_index,
                    start: index - str.len(), 
                    end: index - 1, 
                    number: num
                })
            }
            (_, _) => {}
        }
    }

    if current_chars.len() >= 1 {
        let str: String = current_chars.into_iter().collect();
        let num: i32 = str.parse().expect("failed to convert number");
        numbers.push(NumberCandidate { 
            row: row_index,
            start: row.len() - str.len(), 
            end: row.len() - 1, 
            number: num
        })
    }

    numbers
}

fn get_numbers(grid: &Vec<Vec<char>>) -> Vec<NumberCandidate> {
    grid.iter()
        .enumerate()
        .flat_map(|(index, row)| get_row_numbers(row, index))
        .collect()
}

fn increment_unless_exceeds(num: usize, limit: usize) -> Option<usize> {
    if num + 1 <= limit {
        Some(num + 1)
    } else {
        None
    }
}

fn neighbors(x: usize, y: usize, grid_width: usize, grid_height: usize) -> impl Iterator<Item=(usize, usize)> {
    [
        (x.checked_sub(1), y.checked_sub(1)), 
        (Some(x), y.checked_sub(1)), 
        (increment_unless_exceeds(x, grid_width - 1), y.checked_sub(1)),
        (x.checked_sub(1), Some(y)),                 
        (increment_unless_exceeds(x, grid_width - 1), Some(y)),
        (x.checked_sub(1), increment_unless_exceeds(y, grid_height - 1)), 
        (Some(x), increment_unless_exceeds(y, grid_height - 1)), 
        (
            increment_unless_exceeds(x, grid_width - 1), 
            increment_unless_exceeds(y, grid_height - 1)
        )
    ].into_iter()
    .filter_map(|tuple| {
        match tuple {
            (Some(x), Some(y)) => Some((x, y)),
            (_, _) => None
        }
    })
}

fn check_number(number_candidate: &NumberCandidate, grid: &Vec<Vec<char>>) -> bool {
    (number_candidate.start..=number_candidate.end).any(| col | {
        neighbors(
            col, 
            number_candidate.row, 
            grid[0].len(), 
            grid.len()
        ).any(|neighbor| {
            match grid[neighbor.1][neighbor.0] {
                '0'..='9' => false,
                '.' => false,
                _ => true
            }  
        })
    })
}

fn find_part_numbers(number_candidate: Vec<NumberCandidate>, grid: &Vec<Vec<char>>) -> Vec<NumberCandidate> {
    number_candidate
        .into_iter()
        .filter(|number_candidate| check_number(number_candidate, grid))
        .collect()
}

fn part1() -> i32 {
    let grid: Vec<Vec<char>> = get_puzzle_input()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    
    let numbers = get_numbers(&grid);
    find_part_numbers(numbers, &grid)
        .iter()
        .map(|number| number.number)
        .sum()
}

fn get_gears_candidates(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row
                .iter()
                .enumerate()
                .filter_map(move |(x, char)| {
                    if *char == '*' {
                        Some((x, y))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

fn get_gear_ratios(gear_candidates: Vec<(usize, usize)>, part_numbers: Vec<NumberCandidate>) -> Vec<i32> {
    gear_candidates
        .into_iter()
        .filter_map(|(x, y)| {
            let adjacent_parts: Vec<&NumberCandidate> = part_numbers
                .iter()
                .filter(|part| {
                    neighbors(x, y, usize::MAX, usize::MAX)
                        .any(|(x, y)| {
                            x >= part.start && x <= part.end && y == part.row
                        })
                })
                .collect();
            
            if adjacent_parts.len() >= 2 {
                Some(adjacent_parts
                        .iter()
                        .map(|part| part.number)
                        .product()
                )
            } else {
                None
            }
                
        })
        .collect()
}

fn part2() -> i32 {
    let grid: Vec<Vec<char>> = get_puzzle_input()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    
    let numbers = get_numbers(&grid);
    let part_numbers = find_part_numbers(numbers, &grid);

    let gears_candidates = get_gears_candidates(&grid);
    get_gear_ratios(gears_candidates, part_numbers)
        .iter()
        .sum()


}