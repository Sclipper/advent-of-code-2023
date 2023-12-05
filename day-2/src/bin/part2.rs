use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn pick_number(word: &str) -> i32 {
    let mut number = String::new();
    for character in word.chars() {
        if character.is_digit(10) {
            number.push(character);
        }
    }

    if number.len() == 0 {
        return 1;
    }

    number.parse::<i32>().unwrap()
}

fn parse_input(input: &str) -> Vec<HashMap<String, i32>> {
    let mut parsed_input: Vec<HashMap<String, i32>> = Vec::new();
    for line in input.lines() {
        let mut game: i32 = 0;
        let mut blue: i32 = 0;
        let mut red: i32 = 0;
        let mut green: i32 = 0;
        let mut game_data: HashMap<String, i32> = HashMap::new();
        for word in line.split(":") {
            if word.contains("Game") {
                game = word
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
            } else {
                for color_word_group in word.split(";") {
                    for color_word in color_word_group.split(",") {
                        let number = pick_number(color_word);
                        if color_word.contains("blue") {
                            if number > blue {
                                blue = pick_number(color_word);
                            }
                        } else if color_word.contains("red") {
                            if number > red {
                                red = pick_number(color_word);
                            }
                        } else if color_word.contains("green") {
                            if number > green {
                                green = pick_number(color_word);
                            }
                        }
                    }
                }
            }
        }

        game_data.insert("game".to_string(), game);
        game_data.insert("blue".to_string(), blue);
        game_data.insert("red".to_string(), red);
        game_data.insert("green".to_string(), green);
        parsed_input.push(game_data);
    }


        parsed_input
}

fn part1(input: &str) {

    let parsed_input = parse_input(input);

    // Multiply all the cube numbers together
    let mut sum = 0;

    for game in &parsed_input {
        sum += game["blue"] * game["red"] * game["green"];
    }
    println!("{:?}", sum);
}
