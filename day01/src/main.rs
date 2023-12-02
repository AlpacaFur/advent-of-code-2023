use std::fs;

use regex::{Regex, Match};

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
    let contents = fs::read_to_string("broken_case")
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



fn line_to_number_including_hybrids(line: &str) -> i32 {
    let regex = Regex::new("\\d|zero|one|two|three|four|five|six|seven|eight|nine")
    .expect("expected to be a valid regex");

    let first_match = regex.find(line)
        .expect("expected at least one number");

    let first = string_to_num_string(first_match.as_str());

    let first_index = first_match.start();
    let last_matches_excluding_first = regex.find_iter(&line[first_index + 1..]);

    let last = last_matches_excluding_first
        .last()
        .as_ref()
        .map(Match::as_str)
        .map(string_to_num_string)
        .unwrap_or(first.to_owned());

    let num = first + &last;

    num.parse()
        .expect("expected match to be a number")
}