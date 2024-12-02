fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    run_part1(&contents);
    run_part2(&contents);
}

fn run_part1(contents: &str) {
    let mut left_numbers= Vec::<usize>::new();
    let mut right_numbers= Vec::<usize>::new();

    contents
        .split_terminator("\n")
        .for_each(|it| {            
            let numbers: Vec<&str> = it.split_whitespace().collect();
            assert!(numbers.len() == 2);
            let left: usize = numbers[0].parse().expect("Failed to parse left side number");
            let right: usize = numbers[1].parse().expect("Failed to parse right side number");
            left_numbers.push(left);
            right_numbers.push(right);
        });

    assert!(left_numbers.len() == 1000);
    assert!(right_numbers.len() == 1000);

    left_numbers.sort();
    right_numbers.sort();

    let mut total_distance: usize = 0;
    for (idx, value) in left_numbers.iter().enumerate() {
        total_distance += value.abs_diff(right_numbers[idx])
    }

    println!("Part 1: {total_distance}");
}

fn run_part2(contents: &str) {
    use std::collections::HashMap;

    let mut left_numbers= Vec::<usize>::new();
    let mut counters: HashMap<usize, usize> = HashMap::new();

    contents
        .split_terminator("\n")
        .for_each(|it| {            
            let numbers: Vec<&str> = it.split_whitespace().collect();
            assert!(numbers.len() == 2);
            let left: usize = numbers[0].parse().expect("Failed to parse left side number");
            let right: usize = numbers[1].parse().expect("Failed to parse right side number");
            left_numbers.push(left);

            let current_counter: usize = match counters.get(&right) {
                Some(v) => *v,
                None => 0
            };
            counters.insert(right, current_counter + 1);
        });

    assert!(left_numbers.len() == 1000);

    left_numbers.sort();

    let mut total_distance: usize = 0;
    for (_, value) in left_numbers.iter().enumerate() {
        let default_value: usize = 0;
        let right_appearances = counters.get(value).unwrap_or(&default_value);
        total_distance += value * *right_appearances;
    }

    println!("Part 2: {total_distance}");
}
