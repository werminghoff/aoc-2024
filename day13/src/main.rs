#[derive(Debug)]
struct Coordinates {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
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

mod part1 {

    pub fn run(contents: &str) {
        let result = 0;
        let input = super::parse(contents);
        println!("Part 1 result: {result}");
    }

}

mod part2 {

    pub fn run(contents: &str) {
        let result = 0;
        let input = super::parse(contents);
        println!("Part 2 result: {result}");
    }
    
}