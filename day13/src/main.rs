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
    let file_path = "input/input-sample.txt";
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
    //const ADDED_VALUE: usize = 10000000;

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

        // for machine in input {
        //     min_tokens += calc_min_tokens(&machine);
        // }

        calc_min_tokens(&input[0]);

        println!("Part 2 result: {min_tokens}");

    }

    fn calc_min_tokens(machine: &ClawMachine) -> usize {
        let button_a = &machine.offset_btn_a;
        let button_b = &machine.offset_btn_b;
        let prize = &machine.prize_position;

        let mut b_presses: usize = (prize.x / button_b.x).min(prize.y / button_b.y);
        let mut a_presses: usize = 0;

        loop {
            let total_x = button_b.x * b_presses + button_a.x * a_presses;
            let total_y = button_b.y * b_presses + button_a.y * a_presses;

            // println!("Presses: [a={a_presses}] [b={b_presses}]");
            // println!("TotalX: {total_x}");
            // println!("TotalY: {total_y}");
            // println!("");

            if total_x == prize.x && total_y == prize.y {
                return a_presses * super::PRICE_BUTTON_A + b_presses * super::PRICE_BUTTON_B;
            } else if total_x > prize.x || total_y > prize.y {

                if b_presses == 0 {
                    return 0
                }

                b_presses -= 1;
            } else {
                a_presses += 1;
            }
        }
        

        // (button_a.x * a_presses) + (button_b.x * b_presses) == prize.x
        // (button_a.y * a_presses) + (button_b.y * b_presses) == prize.y
    }

}