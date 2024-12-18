#[derive(Debug, Clone)]
struct Coordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
struct ClawMachine {
    pub offset_btn_a: Coordinates,
    pub offset_btn_b: Coordinates,
    pub prize_position: Coordinates,
}

fn parse_button(content: &str) -> Coordinates {
    let values = &content[10..];
    let values: Vec<&str> = values.split(", ").collect();
    let x_val = &values[0][2..];
    let y_val = &values[1][2..];    
    Coordinates{
        x: x_val.parse().unwrap(),
        y: y_val.parse().unwrap()
    }
}

fn parse_prize(content: &str) -> Coordinates {
    let values = &content[7..];
    let values: Vec<&str> = values.split(", ").collect();
    let x_val = &values[0][2..];
    let y_val = &values[1][2..];

    Coordinates{
        x: x_val.parse().unwrap(),
        y: y_val.parse().unwrap()
    }
}

fn parse(contents: &str) -> Vec<ClawMachine> {
    contents.split("\n\n").map(|machine| {
        let splitted: Vec<&str> = machine.split("\n").map(|line| line).collect();
        assert_eq!(splitted.len(), 3);
        ClawMachine {
            offset_btn_a: parse_button(splitted[0]),
            offset_btn_b: parse_button(splitted[1]),
            prize_position: parse_prize(splitted[2]),
        }
    }).collect()
}

fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    part2::run(&contents);
}

const PRICE_BUTTON_A: usize = 3;
const PRICE_BUTTON_B: usize = 1;

mod part1 {

    struct MinimumConfig {
        pub tokens: usize,
        pub presses_a: usize,
        pub presses_b: usize
    }

    pub fn run(contents: &str) {
        let input = super::parse(contents);
        
        let mut min_tokens: usize = 0;

        for machine in input {
            let button_a = machine.offset_btn_a;
            let button_b = machine.offset_btn_b;
            let prize = machine.prize_position;

            let limit_a: usize = 100;
            let limit_b: usize = 100;

            let mut machine_minimum: Option<MinimumConfig> = None;

            for a_count in 0..limit_a {

                let total_x_a = a_count * button_a.x as usize;
                if total_x_a > prize.x {
                    break;
                }

                let total_y_a = a_count * button_a.y as usize;
                if total_y_a > prize.y {
                    break;
                }

                for b_count in 0..limit_b {

                    let total_x = total_x_a + (b_count * button_b.x as usize);
                    let total_y = total_y_a + (b_count * button_b.y as usize);
                    if total_x > prize.x {
                        break;
                    }
                    if total_y > prize.y {
                        break;
                    }

                    if total_x == prize.x && total_y == prize.y {

                        let next_min = MinimumConfig {
                            presses_a: a_count,
                            presses_b: b_count,
                            tokens: (a_count * super::PRICE_BUTTON_A) + (b_count * super::PRICE_BUTTON_B)
                        };
    
                        if let Some(current_min) = &machine_minimum {
                            if current_min.tokens > next_min.tokens {
                                machine_minimum = Some(next_min)
                            }
                        } else {
                            machine_minimum = Some(next_min)
                        }

                    }
                    
                
                }

            }
        
            if let Some(machine_minimum) = &machine_minimum {
                min_tokens += machine_minimum.tokens;
            }
        }

        println!("Part 1 result: {min_tokens}");

    }

}

mod part2 {
    use crate::ClawMachine;

    const ADDED_VALUE: usize = 10000000000000;

    pub fn fix_added_value(input: &Vec<ClawMachine>) -> Vec<ClawMachine> {
        input.iter().map(|it| {
            let mut machine = it.clone();
            machine.prize_position = super::Coordinates {
                x: machine.prize_position.x + ADDED_VALUE,
                y: machine.prize_position.y + ADDED_VALUE,
            };
            machine
        }).collect()
    }

    pub fn run(contents: &str) {
        let input = super::parse(contents);
        let input = fix_added_value(&input);

        let mut min_tokens: usize = 0;

        let results: Vec<usize> = input.iter().map(|it| {
            calc_min_tokens(it)
        }).collect();

        for res in results {
            min_tokens += res;
        }

        println!("Part 2 result: {min_tokens}");
    }

    fn calc_min_tokens(machine: &ClawMachine) -> usize {
        let button_a = &machine.offset_btn_a;
        let button_b = &machine.offset_btn_b;
        let prize = &machine.prize_position;
        
        let ax = button_a.x as i64;
        let ay = button_a.y as i64;

        let bx = button_b.x as i64;
        let by = button_b.y as i64;

        let px = prize.x as i64;
        let py = prize.y as i64;

        // thx 3Blue1Brown @ YT
        // https://www.youtube.com/watch?v=jBsC34PxzoM

        let det_area = ax * by - ay * bx;
        let det_a = px * by - py * bx;
        let det_b = py * ax - px * ay;

        if det_a % det_area == 0 && det_b % det_area == 0 {
            let price_a = super::PRICE_BUTTON_A as i64 * det_a / det_area;
            let price_b = super::PRICE_BUTTON_B as i64 * det_b / det_area;
            (price_a + price_b) as usize
        } else {
            0
        }
    }

}