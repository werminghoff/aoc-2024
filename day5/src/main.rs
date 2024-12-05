#[derive(Debug)]
struct PageOrderingRule {
    pub first: usize,
    pub second: usize
}

fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    part2::run(&contents);
}

fn is_breaking_rule(rule: &PageOrderingRule, update: &Vec<usize>) -> bool {
    let mut pages_done = Vec::<usize>::new();
    pages_done.reserve(update.len());

    for page in update {            

        if pages_done.contains(&rule.second) && *page == rule.first {
            // let first = &rule.first;
            // let second = &rule.second;
            //println!("Update {update:?} breaks rule [{first}|{second}] with pages_done {pages_done:?}");
            return true;
        }                
    
        pages_done.push(*page);
    }
    
    false
}

fn is_update_in_right_order(update: &Vec<usize>, rules: &Vec<PageOrderingRule>) -> bool {
        
    let mut pages_done = Vec::<usize>::new();
    pages_done.reserve(update.len());

    for page in update {            

        let page_rules: Vec<&PageOrderingRule> = rules.iter().filter(|it| it.first == *page).collect();

        for rule in page_rules {                
            if pages_done.contains(&rule.second) {
                return false;
            }                
        }

        pages_done.push(*page);
    }
    
    true
}

mod part1 {

    use super::PageOrderingRule;

    pub fn run(contents: &str) {

        let mut ordering= Vec::<PageOrderingRule>::new();
        
        let mut updates= Vec::<Vec<usize>>::new();

        let mut is_updates = false;

        contents
            .split_terminator("\n")
            .for_each(|it| {            
                if it.is_empty() {
                    is_updates = true;
                    return;
                }

                if is_updates {
                    let numbers = it.split(",").map(|it| { it.parse().expect("Failed to parse update number") }).collect();
                    updates.push(numbers);                   
                } else {
                    let numbers: Vec<&str> = it.split("|").collect();
                    let first = numbers[0].parse().expect("Failed to parse second number");
                    let second = numbers[1].parse().expect("Failed to parse second number");
                    ordering.push(PageOrderingRule { first: first, second: second });
                }
            });

        // println!("Ordering: {ordering:?}");
        // println!("Numbers: {updates:?}");

        let correct_updates: Vec<&Vec<usize>> = updates.iter().filter(|it| {
            super::is_update_in_right_order(it, &ordering)
        }).collect();

        //let count = correct_updates.len();
        //println!("Correct updates [{count}]: {correct_updates:?}");
        
        let middle_numbers: Vec<usize> = correct_updates.iter().map(|it| {
            let len = it.len() as f64;
            let idx = ((len as f64 / 2.0).ceil() as usize) - 1;
            it[idx]
        }).collect();

        //println!("Middle numbers: {middle_numbers:?}");
        let mut count = 0;
        middle_numbers.iter().for_each(|it| count += it);
        println!("Part 1 result: {count}");        
    }
    
}

mod part2 {
    use super::PageOrderingRule;

    pub fn run(contents: &str) {
        let mut ordering= Vec::<PageOrderingRule>::new();
        
        let mut updates= Vec::<Vec<usize>>::new();

        let mut is_updates = false;

        contents
            .split_terminator("\n")
            .for_each(|it| {            
                if it.is_empty() {
                    is_updates = true;
                    return;
                }

                if is_updates {
                    let numbers = it.split(",").map(|it| { it.parse().expect("Failed to parse update number") }).collect();
                    updates.push(numbers);                   
                } else {
                    let numbers: Vec<&str> = it.split("|").collect();
                    let first = numbers[0].parse().expect("Failed to parse second number");
                    let second = numbers[1].parse().expect("Failed to parse second number");
                    ordering.push(PageOrderingRule { first: first, second: second });
                }
            });

        // println!("Ordering: {ordering:?}");
        // println!("Numbers: {updates:?}");

        let incorrect_updates: Vec<&Vec<usize>> = updates.iter().filter(|it| {
            !super::is_update_in_right_order(it, &ordering)
        }).collect();

        //let count = incorrect_updates.len();
        //println!("Incorrect updates [{count}]: {incorrect_updates:?}");

        let fixed_updates: Vec<Vec<usize>> = incorrect_updates.iter().map(|it| {
            fix_ordering(it, &ordering)
        }).collect();

        //let count = fixed_updates.len();
        //println!("Fixed updates [{count}]: {fixed_updates:?}");

        let middle_numbers: Vec<usize> = fixed_updates.iter().map(|it| {
            let len = it.len() as f64;
            let idx = ((len as f64 / 2.0).ceil() as usize) - 1;
            it[idx]
        }).collect();

        //println!("Middle numbers: {middle_numbers:?}");
        let mut count = 0;
        middle_numbers.iter().for_each(|it| count += it);
        println!("Part 2 result: {count}");

    }

    fn fix_ordering(update: &Vec<usize>, rules: &Vec<PageOrderingRule>) -> Vec<usize> {
        let mut fixed_updates = update.clone();
        
        loop {
            // probably redundant, but whatever
            if super::is_update_in_right_order(&fixed_updates, &rules) {
                break
            }

            for rule in rules {
                if super::is_breaking_rule(rule, &fixed_updates) {
                    // println!("");
                    // println!("Rule: {rule:?}");
                    // println!("Before: {fixed_updates:?}");
                    apply_rule(rule, &mut fixed_updates);
                    // println!("After: {fixed_updates:?}");
                    // println!("");
                }
            }
        }
        
        fixed_updates
    }

    fn apply_rule(rule: &PageOrderingRule, updates: &mut Vec<usize>) {
        let Some(first_idx) = updates.iter().position(|it| *it == rule.first) else {
            return
        };
        let Some(second_idx) = updates.iter().position(|it| *it == rule.second) else {
            return
        };

        let val = updates.remove(second_idx);
        if updates.len() > first_idx {
            updates.insert(first_idx+1, val);
        } else {
            updates.push(val);
        }
    }

}