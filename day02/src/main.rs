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

struct CubeCount {
    red: i32,
    green: i32,
    blue: i32
}

struct Game {
    id: i32,
    rounds: Vec<CubeCount>
}

fn row_to_game(row: &str) -> Game {
    let game: Vec<&str> = row.split(": ").collect();
    let id: i32 = game[0][5..].parse().expect("game id should be a number");
    let rounds: Vec<CubeCount> = game[1]
        .split(";")
        .map(|round_str| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            round_str.split(", ").for_each(|color_count_str| {
                let color_count = color_count_str.trim().split(" ").collect::<Vec<&str>>();
                let count: i32 = color_count[0].parse().expect("expected valid number");
                let color = color_count[1];
                match color {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => panic!("expected valid color")
                }
            });

            CubeCount {
                red,
                green,
                blue
            }
        }).collect();

    Game {
        id,
        rounds
    }
}

fn game_is_possible(game: &Game) -> bool {
    let CubeCount {red, green, blue} = min_cubes(game);

    red <= 12 && green <= 13 && blue <= 14
}

fn min_cubes(game: &Game) -> CubeCount {
    let red = game.rounds
        .iter()
        .map(|round| round.red)
        .max()
        .unwrap_or(0);
    let green = game.rounds
        .iter()
        .map(|round| round.green)
        .max()
        .unwrap_or(0);
    let blue = game.rounds
        .iter()
        .map(|round| round.blue)
        .max()
        .unwrap_or(0);

    CubeCount { red, green, blue }
}

fn part1() -> i32 {
    get_puzzle_input()
        .iter()
        .map(|s| row_to_game(s))
        .filter(game_is_possible)
        .map(|game| game.id)
        .sum()
}

fn part2() -> i32 {
    get_puzzle_input()
        .iter()
        .map(|s| row_to_game(s))
        .map(|g| min_cubes(&g))
        .map(|g| g.red * g.green * g.blue)
        .sum()
}