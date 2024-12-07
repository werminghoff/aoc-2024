use std::str::FromStr;

fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    part2::run(&contents);
}

#[derive(Debug, Clone, Default)]
struct InputLine {
    pub result: usize,
    pub numbers: Vec<usize>
}

impl FromStr for InputLine {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> { 
        
        let parts: Vec<&str> = line.split(":").collect();
        let result: usize = parts[0].parse().unwrap();
        let numbers: Vec<usize> = parts[1].split_whitespace()
                                                .filter(|it| !it.is_empty())
                                                .map(|it| it.parse().unwrap())
                                                .collect();

        let res = InputLine {
            result: result,
            numbers: numbers
        };

        Ok(res)
    }
}

mod part1 {
    use super::*;

    #[derive(Debug, Clone)]
    enum Operation {
        Add,
        Multiply
    }

    impl Operation {

        fn first() -> Self {
            Operation::Add
        }

        fn next(&self) -> Option<Self> {
            match self {
                Operation::Add => Some(Operation::Multiply),
                Operation::Multiply => None
            }
        }

    }

    pub fn run(contents: &str) {

        let all_lines: Vec<InputLine> = contents.split("\n")
                                            .map(|it| {
                                                InputLine::from_str(it).unwrap()
                                            }).collect();
        
        let mut counter: usize = 0;

        for line in all_lines {
            if is_valid_line(&line) {
                counter += line.result;
            }
        }

        println!("Part 1 result: {counter}");

    }

    fn is_valid_line(line: &InputLine) -> bool {
        let mut operators = Vec::<Operation>::new();

        for _ in 0..line.numbers.len() - 1 {
            operators.push(Operation::first());
        }

        loop {
            //println!("Trying operations: {operators:?}");
            let res = calculate(&line.numbers, &operators);
            if res == line.result {
                return true;
            }

            let mut next_update_idx: usize = 0;
            loop {                
                match operators[next_update_idx].next() {
                    Some(v) => {
                        operators[next_update_idx] = v;
                        break;
                    },
                    None => {
                        operators[next_update_idx] = Operation::first();
                        next_update_idx += 1;
                        if next_update_idx >= operators.len() {
                            return false;
                        }
                    }
                }
            }

        }
    }

    fn calculate(numbers: &Vec<usize>, operators: &Vec<Operation>) -> usize {

        let mut res: usize = numbers[0];

        let numbers_len = numbers.len();

        for idx in 0..numbers_len-1 {

            res = match operators[idx] {
                Operation::Add => res + numbers[idx+1],
                Operation::Multiply =>  res * numbers[idx+1],
            };

        }

        res
    }

}

mod part2 {
    use super::*;

    #[derive(Debug, Clone)]
    enum Operation {
        Add,
        Multiply,
        Concat
    }

    impl Operation {

        fn first() -> Self {
            Operation::Add
        }

        fn next(&self) -> Option<Self> {
            match self {
                Operation::Add => Some(Operation::Multiply),
                Operation::Multiply => Some(Operation::Concat),
                Operation::Concat => None
            }
        }

    }

    pub fn run(contents: &str) {
        let all_lines: Vec<InputLine> = contents.split("\n")
                                            .map(|it| {
                                                InputLine::from_str(it).unwrap()
                                            }).collect();
        
        let mut counter: usize = 0;

        for line in all_lines {
            if is_valid_line(&line) {
                counter += line.result;
            }
        }

        println!("Part 2 result: {counter}");
    }

    fn is_valid_line(line: &InputLine) -> bool {
        let mut operators = Vec::<Operation>::new();

        for _ in 0..line.numbers.len() - 1 {
            operators.push(Operation::first());
        }

        loop {
            //println!("Trying operations: {operators:?}");
            let res = calculate(&line.numbers, &operators);
            if res == line.result {
                return true;
            }

            let mut next_update_idx: usize = 0;
            loop {                
                match operators[next_update_idx].next() {
                    Some(v) => {
                        operators[next_update_idx] = v;
                        break;
                    },
                    None => {
                        operators[next_update_idx] = Operation::first();
                        next_update_idx += 1;
                        if next_update_idx >= operators.len() {
                            return false;
                        }
                    }
                }
            }

        }
    }

    fn calculate(numbers: &Vec<usize>, operators: &Vec<Operation>) -> usize {

        let mut res: usize = numbers[0];

        let numbers_len = numbers.len();

        for idx in 0..numbers_len-1 {

            res = match operators[idx] {
                Operation::Add => res + numbers[idx+1],
                Operation::Multiply =>  res * numbers[idx+1],
                Operation::Concat => {
                    let next_number = numbers[idx+1];
                    format!("{res}{next_number}").parse().unwrap()
                }
            };

        }

        res
    }

}