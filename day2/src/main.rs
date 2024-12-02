
#[derive(Clone, Debug, PartialEq)]
enum Direction {
    INCREASING,
    DECREASING
}

#[derive(PartialEq, Debug)]
enum Safety {
    SAFE,
    UNSAFE
}

fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    run_part1(&contents);
    run_part2(&contents);
}

fn run_part1(contents: &str) {
    let report_safety: Vec<Safety> = contents
    .split_terminator("\n")
    .map(|it| {            
        let numbers: Vec<usize> = it.split_whitespace()
            .flat_map(|it| it.parse())
            .collect();

        is_safe_report(&numbers)
    })
    .collect();

    let safe_count = report_safety.iter()
                                        .filter(|it| {
                                            **it == Safety::SAFE
                                        }).count();
    println!("Safe reports: {safe_count}");
}

fn run_part2(contents: &str) {
    let report_safety: Vec<Safety> = contents
    .split_terminator("\n")
    .map(|it| {            
        let numbers: Vec<usize> = it.split_whitespace()
            .flat_map(|it| it.parse())
            .collect();

        for (idx, _) in numbers.iter().enumerate() {
            let mut mutable_numbers = numbers.clone();
            mutable_numbers.remove(idx);
            if is_safe_report(&mutable_numbers) == Safety::SAFE {
                return Safety::SAFE;
            }
        }
        
        Safety::UNSAFE
    })
    .collect();

    let safe_count = report_safety.iter()
                                        .filter(|it| {
                                            **it == Safety::SAFE
                                        }).count();
    println!("Safe reports: {safe_count}");
}

fn is_safe_report(numbers: &Vec<usize>) -> Safety {
    let mut direction: Option<Direction> = None;
        
    let cols = numbers.len();
    for (idx, value) in numbers.iter().enumerate() {
        if idx == cols-1 {
            continue;
        }

        let lhs = *value;
        let rhs = numbers[idx+1]; 
        if lhs == rhs {
            return Safety::UNSAFE;
        }

        if direction.is_none() {
            if lhs < rhs {
                direction = Some(Direction::INCREASING);
            } else {
                direction = Some(Direction::DECREASING);
            }
        } else {
            let current_direction = direction.clone().expect("Direction should be available at this point");
            if lhs < rhs {
                if current_direction != Direction::INCREASING {
                    return Safety::UNSAFE;
                }
            } else {
                if current_direction != Direction::DECREASING {
                    return Safety::UNSAFE;
                }
            }
        }

        if lhs.abs_diff(rhs) > 3 {
            return Safety::UNSAFE;
        }
    }
    
    Safety::SAFE
}