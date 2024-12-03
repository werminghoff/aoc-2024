use regex::Regex;

struct MatchInfo {
    pub length: usize,
    pub first_value: usize,
    pub second_value: usize
}

fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    run_part1(&contents);
    run_part2(&contents);
}

fn run_part1(contents: &str) {
    let rgx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Failed to parse regex pattern");
    let mut sum: usize = 0;
    for (matched, [_, _]) in rgx.captures_iter(contents).map(|c| c.extract()) {
        let Some(pair) = try_parse_multiplication(matched) else {
            return;
        };
        sum += pair.0 * pair.1;
    }

    println!("Part 1 result: {sum}");
}

fn run_part2(contents: &str) {

    let mut start_idx: usize = 0;
    let mut is_enabled = true;
    let mut total_sum: usize = 0;

    const DO_OP: &str = "do()";
    const DONT_OP: &str = "don't()";
    
    loop {
        if contents.len() == start_idx + 1 {
            break;
        }

        // check if it for "do()"
        if is_operation(contents, DO_OP, start_idx) {
            is_enabled = true;
            start_idx += DO_OP.len();
            continue;
        }

        // check if it for "don't()"
        if is_operation(contents, DONT_OP, start_idx) {
            is_enabled = false;
            start_idx += DONT_OP.len();
            continue;
        }

        if !is_enabled {
            start_idx += 1;
            continue;
        }

        if let Some(matched) = find_multiplication(contents, start_idx) {

            //println!("found multiplication at {start_idx}");
            
            total_sum += matched.first_value * matched.second_value;
            start_idx += matched.length;
            continue;
        }
        
        start_idx += 1;
    }

    println!("Part 2 result: {total_sum}");
    
}

fn is_operation(contents: &str, operation: &str, start_idx: usize) -> bool {
    let end = start_idx + operation.len();
    if contents.len() < end {
        return false
    }
    let sliced = &contents[start_idx..end];
    return sliced == operation
}

fn find_multiplication(contents: &str, start_idx: usize) -> Option<MatchInfo> {
    if !&contents[start_idx..].starts_with("mul(") {
        return None;
    }

    const MIN_LEN: usize = "mul(1,2)".len();
    const MAX_LEN: usize = "mul(111,233)".len() + 1;

    for i in MIN_LEN..MAX_LEN {
        let end = start_idx + i;
        if contents.len() < end {
            return None;
        }
        let sliced = &contents[start_idx..end];
        if let Some(pair) = try_parse_multiplication(sliced) {
            return Some(MatchInfo {
                length: sliced.len(),
                first_value: pair.0,
                second_value: pair.1
            })
        }
    }

    None
}

fn try_parse_multiplication(value: &str) -> Option<(usize, usize)> {
    let rgx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Failed to parse regex pattern");
    for (_, [first_num, second_num]) in rgx.captures_iter(value).map(|c| c.extract()) {
        let first_num: usize = first_num.parse().expect("Failed to parse first number");
        let second_num: usize = second_num.parse().expect("Failed to parse second number");
        return Some((first_num, second_num))
    }
    return None
}