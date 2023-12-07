fn main() {
    let input = include_str!("./input1.txt");

    let result = part1(input);

    println!("{result}");
}
fn extract_numbers(string: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for word in string.split(" ").filter(|word| !word.contains(":")) {
        if !word.is_empty() && word.chars().all(char::is_numeric) {
            numbers.push(word.parse::<u32>().unwrap());
        }
    }

    return numbers;
}

fn part1(input: &str) -> i32 {  
    let mut total_points = 0;

    for line in input.split("\n") {
        let parts: Vec<&str> = line.split("|").collect();
        let winning_numbers = extract_numbers(parts[0]);
        let elf_numbers = extract_numbers(parts[1]);
        let numbers_hit: Vec<u32> = winning_numbers
            .iter()
            .filter(|winning_number| elf_numbers.contains(winning_number))
            .map(|winning_number| *winning_number)
            .collect();
        println!("numbers_hit {:?}", numbers_hit);
        let points = numbers_hit.iter().fold(0, |acc, _| {
            if acc == 0 {
                return 1;
            }
            acc * 2
        });
        total_points += points;
    }

    return total_points;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(13, part1(input));
    }
}
