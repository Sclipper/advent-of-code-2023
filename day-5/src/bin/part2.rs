use rayon::prelude::*;


fn main() {
    let input = include_str!("./input1.txt");

    let result = part1(input);

    println!("{result}");
}

// fn get_seeds(input: &str) -> Vec<i64> {
//     let collected_lines: Vec<&str> = input.lines().collect();
//     let splits: Vec<&str> = collected_lines[0].split(":").collect();
//     let numbers: Vec<&str> = splits[1].split(" ").collect();
//     let seed_numbers: Vec<i64> = numbers
//         .iter()
//         .filter(|num| !num.is_empty())
//         .map(|number| {
//             let num = number.parse::<i64>().unwrap();
//             return num;
//         })
//         .collect();

//     let mut starting_numbers: Vec<i64> = Vec::new();
//     let mut ranges: Vec<i64> = Vec::new();
    
//     for (i, num) in seed_numbers.iter().enumerate() {
//         if i % 2 != 0 {
//             ranges.push(*num);
//             continue;
//         }
//         starting_numbers.push(*num)
//     }
    
//     let mut final_numbers: Vec<i64> = Vec::new();
//     for (i, num) in starting_numbers.iter().enumerate() {
//         let range_val = ranges[i];
//         for j in 0..range_val {
//             final_numbers.push(num + j);
//         }
//     }
//     final_numbers
// }

fn get_seeds(input: &str) -> Vec<i64> {
    let line = input.lines().next().unwrap();
    let numbers: Vec<i64> = line.split(":").nth(1).unwrap().split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let mut final_numbers: Vec<i64> = Vec::new();
    for (i, &num) in numbers.iter().enumerate() {
        if i % 2 == 0 {
            let range_val = numbers[i + 1];
            for j in 0..range_val {
                final_numbers.push(num + j);
            }
        }
    }
    final_numbers
}

fn get_line_numbers(input: &str, step: &str) -> Vec<Vec<i64>> {
    let blocks = input.split("\n\n").filter(|block| block.contains(step));
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    for block in blocks {
        let lines = block.lines();
        for line in lines {
            if line.starts_with(step) {
                continue;
            }
            let num_array = line
                .split_whitespace()
                .filter(|num| !num.is_empty())
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            numbers.push(num_array);
        }
    }
    return numbers;
}

fn find_line(lines: Vec<Vec<i64>>, source: i64) -> Option<Vec<i64>> {
    lines
        .into_iter()
        .find(|line| source >= line[1] && source <= line[1] + line[2])
}

fn get_step_map(input: &str, seed: i64) -> Vec<i64> {
    let steps = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];
    let mut seed_map: Vec<i64> = Vec::new();
    seed_map.push(seed);
    let mut source = seed;
    for step in steps {
        let lines: Vec<Vec<i64>> = get_line_numbers(input, step);

        let line = find_line(lines, source);
        match line {
            Some(line) => {
                let line_source = line[1];
                let line_destination = line[0];
                let diff = &line_source - &line_destination;
                let new_source = source - diff;
                source = new_source;
                seed_map.push(new_source);
            }
            None => {}
        }
    }
    seed_map
}

// fn part1(input: &str) -> i64 {
//     println!("Getting seeds");
//     let seeds = get_seeds(input);
//     println!("Got seeds");
//     println!("Making maps maps");
//     let mut maps: Vec<Vec<i64>> = Vec::new();
//     for seed in seeds {
//         let step_map: Vec<i64> = get_step_map(input, seed);
//         maps.push(step_map);
//     }
//     println!("Maps are ready");
//     println!("Finding the smallest number");
//     let smallest_last_number = maps
//     .iter()
//     .filter(|map| !map.is_empty()) // Ignore empty maps
//     .map(|map| map.last().unwrap()) // Get the last number of each map
//     .min() // Find the smallest last number
//     .unwrap_or(&0); // If there are no maps or all maps are empty, return 0

//     *smallest_last_number
// }

fn part1(input: &str) -> i64 {
    println!("Getting seeds");
    let seeds = get_seeds(input);
    println!("Got seeds");
    println!("Making maps maps");

    let maps: Vec<Vec<i64>> = seeds.into_par_iter()
        .map(|seed| get_step_map(input, seed))
        .collect();

    println!("Maps are ready");
    println!("Finding the smallest number");
    let smallest_last_number = maps
        .iter()
        .filter(|map| !map.is_empty()) // Ignore empty maps
        .map(|map| map.last().unwrap()) // Get the last number of each map
        .min() // Find the smallest last number
        .unwrap_or(&0); // If there are no maps or all maps are empty, return 0

    *smallest_last_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
        ";
        assert_eq!(13, part1(input));
    }
}
