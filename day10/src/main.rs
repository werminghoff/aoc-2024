trait ToNumber {
    fn to_number(&self) -> i64;
}

impl ToNumber for char {
    fn to_number(&self) -> i64 {
        if *self >= '0' && *self <= '9' {
            return *self as i64 - '0' as i64;
        }
        panic!("invalid character");
    }
}

fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    part2::run(&contents);
}

type Matrix = Vec<Vec<char>>;

mod part1 {
    use crate::ToNumber;
    use std::collections::HashSet;
    use super::Matrix;

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Position {
        pub x: usize,
        pub y: usize
    }

    pub fn run(contents: &str) {
        let input_matrix: Matrix = parse_input(contents);

        let mut counter: usize = 0;

        for (y, row) in input_matrix.iter().enumerate() {
            for (x, character) in row.iter().enumerate() {

                assert_eq!(input_matrix[y][x], *character);
                if *character != '0' {
                    continue;
                }

                let mut final_pos_set: HashSet<Position> = HashSet::new();

                if visit_node(x as i64, y as i64, 0, &input_matrix, &mut final_pos_set) {
                    counter += final_pos_set.len();
                }

            }
        }

        println!("Part 1 result: {counter}");
    }

    fn parse_input(contents: &str) -> Matrix {
        contents.split_whitespace()
                                        .map(|it| { 
                                            it.chars().collect()
                                         })
                                        .collect()
    }

    fn visit_node(x: i64, y: i64, expected_height: i64, matrix: &Matrix, final_positions: &mut HashSet<Position>) -> bool {
        if y < 0 || x < 0 {
            return false;
        }

        if y >= matrix.len() as i64 {
            return false;
        }

        if x >= matrix[y as usize].len() as i64 {
            return false;
        }

        let character = matrix[y as usize][x as usize]; 
        let current_height = character.to_number();
        
        if current_height != expected_height {
            return false;
        }

        if current_height == 9 {
            let pos = Position {
                x: x as usize,
                y: y as usize
            };
            _ = final_positions.insert(pos);
            return true;
        }

        let left = visit_node(x - 1, y, current_height + 1, matrix, final_positions);
        let right = visit_node(x + 1, y, current_height + 1, matrix, final_positions);
        let top = visit_node(x, y - 1, current_height + 1, matrix, final_positions);
        let bottom = visit_node(x, y + 1, current_height + 1, matrix, final_positions);
        
        left || right || top || bottom
    }

}

mod part2 {
    use crate::ToNumber;
    use std::collections::HashSet;
    use super::Matrix;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Position {
        pub x: usize,
        pub y: usize
    }

    pub fn run(contents: &str) {
        let input_matrix: Matrix = parse_input(contents);

        let mut counter: usize = 0;

        for (y, row) in input_matrix.iter().enumerate() {
            for (x, character) in row.iter().enumerate() {

                assert_eq!(input_matrix[y][x], *character);
                if *character != '0' {
                    continue;
                }

                let mut final_pos_set: HashSet<Vec<Position>> = HashSet::new();

                if visit_node(x as i64, y as i64, 0, &input_matrix, Vec::new(), &mut final_pos_set) {
                    counter += final_pos_set.len();
                }

            }
        }

        println!("Part 2 result: {counter}");
    }

    fn parse_input(contents: &str) -> Matrix {
        contents.split_whitespace()
                                        .map(|it| { 
                                            it.chars().collect()
                                         })
                                        .collect()
    }

    fn visit_node(x: i64, y: i64, expected_height: i64, matrix: &Matrix, current_trail: Vec<Position>, trails: &mut HashSet<Vec<Position>>) -> bool {
        if y < 0 || x < 0 {
            return false;
        }

        if y >= matrix.len() as i64 {
            return false;
        }

        if x >= matrix[y as usize].len() as i64 {
            return false;
        }

        let character = matrix[y as usize][x as usize]; 
        let current_height = character.to_number();
        
        if current_height != expected_height {
            return false;
        }

        let mut current_trail: Vec<Position> = current_trail.clone();
        current_trail.push(Position {
            x: x as usize,
            y: y as usize
        });

        if current_height == 9 {
            _ = trails.insert(current_trail);
            return true;
        }

        let left = visit_node(x - 1, y, current_height + 1, matrix, current_trail.clone(), trails);
        let right = visit_node(x + 1, y, current_height + 1, matrix, current_trail.clone(), trails);
        let top = visit_node(x, y - 1, current_height + 1, matrix, current_trail.clone(), trails);
        let bottom = visit_node(x, y + 1, current_height + 1, matrix, current_trail.clone(), trails);
        
        left || right || top || bottom
    }

}
