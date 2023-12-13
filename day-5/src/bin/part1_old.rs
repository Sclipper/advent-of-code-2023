use std::collections::HashMap;

struct Rec {
    destination: i64,
    source: i64,
    range_len: i64,
    i: i64,
    temp_map: HashMap<i64, i64>,
    largest_num_on_prev_step: i64,
}

fn main() {
    let input = include_str!("./input1.txt");

    let result = part1(input);

    println!("{result}");
}

fn get_seeds(input: &str) -> Vec<i64> {
    let collected_lines: Vec<&str> = input.lines().collect();
    let splits: Vec<&str> = collected_lines[0].split(":").collect();
    let numbers: Vec<&str> = splits[1].split(" ").collect();
    let return_stuff = numbers
        .iter()
        .filter(|num| !num.is_empty())
        .map(|number| {
            let num = number.parse::<i64>().unwrap();
            return num;
        })
        .collect();

    return return_stuff;
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

// fn recursively_add_numbers(rec_struct: Rec) -> HashMap<i64, i64> {
//     let Rec {
//         destination,
//         source,
//         range_len,
//         i,
//         mut temp_map,
//         largest_num_on_prev_step,
//     } = rec_struct;
//     let highest_source = source + range_len;
//     let diff = destination - source;
//     println!("largest_num_on_prev_step, {} i: {}", std::cmp::max(highest_source, largest_num_on_prev_step), i);
    
//     if i == std::cmp::max(highest_source, largest_num_on_prev_step) {
//         return temp_map;
//     }

//     let left = i;
//     let right = if i >= source && i <= highest_source {
//         i + diff
//     } else {
//         i
//     };
//     temp_map.insert(left, right);

//     let updated_rec_struct = Rec {
//         destination,
//         source,
//         range_len,
//         i: i + 1,
//         temp_map,
//         largest_num_on_prev_step,
//     };

//     recursively_add_numbers(updated_rec_struct)
// }
fn recursively_add_numbers(mut rec_struct: Rec) -> HashMap<i64, i64> {
    loop {
        let Rec {
            destination,
            source,
            range_len,
            i,
            mut temp_map,
            largest_num_on_prev_step,
        } = rec_struct;
        let highest_source = source + range_len;
        let diff = destination - source;
    
        if i == std::cmp::max(highest_source, largest_num_on_prev_step) {
            return temp_map;
        }

        let left = i;
        let right = if i >= source && i <= highest_source {
            i + diff
        } else {
            i
        };
        temp_map.insert(left, right);

        rec_struct = Rec {
            destination,
            source,
            range_len,
            i: i + 1,
            temp_map,
            largest_num_on_prev_step,
        };
    }
}

fn get_step_map(input: &str, step: &str, largest_num_on_prev_step: i64) -> HashMap<i64, i64> {
    let mut lines: Vec<Vec<i64>> = get_line_numbers(input, step);
    let mut plantation_map: HashMap<i64, i64> = HashMap::new();
    lines.sort_by(|a, b| a[1].cmp(&b[1]));
    for (index, line) in lines.iter().enumerate() {
        let destination = line[0];
        let source = line[1];
        let range_len = line[2];
 
        let rec_struct = Rec {
            destination,
            source,
            range_len,
            i: if index < 1 { 0 } else { source },
            temp_map: plantation_map.clone(),
            largest_num_on_prev_step,
        };
        plantation_map.extend(recursively_add_numbers(rec_struct));
    }

    plantation_map
}

fn get_last_step_for_each_seed(
    seed: i64,
    steps: [&str; 7],
    maps: &HashMap<&str, HashMap<i64, i64>>,
) -> i64 {
    let mut temp_num = seed;
    for step in steps {
        let step_map = maps.get(step).unwrap();
        temp_num = *step_map.get(&temp_num).unwrap();
    }
    temp_num
}

fn get_largest_num_on_prev_step(
    maps: HashMap<&str, HashMap<i64, i64>>,
    step: &str,
    steps: [&str; 7],
) -> i64 {
    let index_of_cur_step = steps.iter().position(|&r| r == step).unwrap();
    if index_of_cur_step == 0 {
        return 0;
    }
    let prev_step = steps[index_of_cur_step - 1];
    let prev_step_map = maps.get(prev_step).unwrap();
    *prev_step_map.values().max().unwrap_or(&0)
}

fn part1(input: &str) -> i64 {
    let seeds = get_seeds(input);
    let steps = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];
    let mut maps: HashMap<&str, HashMap<i64, i64>> = HashMap::new();
    for step in steps {
        let largest_num_on_prev_step: i64 = get_largest_num_on_prev_step(maps.clone(), step, steps);
        let step_map: HashMap<i64, i64> = get_step_map(input, step, largest_num_on_prev_step);
        maps.insert(step, step_map);
    }

    let mut last_step_for_each_seed: Vec<i64> = Vec::new();
    for seed in seeds {
        last_step_for_each_seed.push(get_last_step_for_each_seed(seed, steps, &maps))
    }
    return *last_step_for_each_seed.iter().min().unwrap_or(&0); 
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_exercise() {
//         let input = "seeds: 79 14 55 13

//         seed-to-soil map:
//         50 98 2
//         52 50 48
        
//         soil-to-fertilizer map:
//         0 15 37
//         37 52 2
//         39 0 15
        
//         fertilizer-to-water map:
//         49 53 8
//         0 11 42
//         42 0 7
//         57 7 4
        
//         water-to-light map:
//         88 18 7
//         18 25 70
        
//         light-to-temperature map:
//         45 77 23
//         81 45 19
//         68 64 13
        
//         temperature-to-humidity map:
//         0 69 1
//         1 0 69
        
//         humidity-to-location map:
//         60 56 37
//         56 93 4
//         ";
//         assert_eq!(13, part1(input));
//     }
// }
