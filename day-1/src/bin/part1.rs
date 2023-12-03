fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) {
    let lines = input.split("\n");
    let mut numbers: Vec<i32> = Vec::new();

    for line in lines {
        let mut line_numbers: Vec<i32> = Vec::new();

        for character in line.chars() {
            if character.is_digit(10) {
                line_numbers.push(character.to_digit(10).unwrap() as i32);
            }
        }

        if line_numbers.len() == 1 {
            line_numbers.push(line_numbers[0]);
        }

        if line_numbers.len() > 2 {
            line_numbers = vec![line_numbers[0], line_numbers[line_numbers.len() - 1]];
        }
        let mut number = line_numbers[0].to_string();
        number.push_str(&line_numbers[1].to_string());
        line_numbers = vec![number.parse::<i32>().unwrap()];
        numbers.push(line_numbers[0]);
    }

    let mut sum = 0;
    for number in &numbers {
        sum += number;
    }
    println!("{:?}", sum);
}
