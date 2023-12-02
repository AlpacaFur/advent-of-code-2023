use std::fs;

fn main() {
    println!("Part 1");
    println!("{}", part1());
    println!("Part 2");
    println!("{}", part2());
}

fn part1() -> i32 {
    let contents = fs::read_to_string("input")
        .expect("missing input file!");
    
    contents
        .lines()
        .map(line_to_number)
        .sum()
}

fn part2() -> i32 {
    let contents = fs::read_to_string("input")
        .expect("missing input file!");
    
    contents
        .lines()
        .map(line_to_number_including_hybrids)
        .sum()
}

fn digit_matcher(char: char) -> bool {
    char >= '0' && char <= '9'
}

fn string_to_num_string(str: &str) -> String {
    match str {
        "zero" => "0",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => str
    }.to_string()
}

fn line_to_number(line: &str) -> i32 {
    let mut match_iter = line.matches(digit_matcher);

    let first = match_iter
        .next()
        .expect("expected at least one digit in the line");

    let last = match_iter
        .next_back()
        .unwrap_or(first);

    let num = first.to_owned() + last;

    num.parse()
        .expect("expected match to be a number")
}


const NUMS: &[&str] = &["zero","one","two","three","four","five","six","seven","eight","nine"];

fn number_starting_at_index(index: usize, char: char, line: &str) -> Option<String> {
    if char.is_ascii_digit() {
        Some(string_to_num_string(&char.to_string()))
    } else {
        NUMS.iter().find_map(|num_str| {
            if line[index..].starts_with(num_str) {
                Some(string_to_num_string(num_str))
            } else {
                None
            }
        })
    } 
}

fn line_to_number_including_hybrids(line: &str) -> i32 {
    let first = line.char_indices().find_map(|(index, char)| {
        number_starting_at_index(index, char, line)
    }).expect("expected at least one number in line");

    let last = line.char_indices().rev().find_map(|(index, char)| {
        number_starting_at_index(index, char, line)
    }).unwrap_or(first.to_owned());

    let num = first + &last;

    num.parse()
        .expect("expected match to be a number")
}