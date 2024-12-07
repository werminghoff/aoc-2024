use std::collections::HashSet;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
struct Position {
    pub x: i64,
    pub y: i64
}

impl Position {

    fn next(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    fn move_up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1
        }
    }

    fn move_down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1
        }
    }

    fn move_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y
        }
    }

    fn move_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y
        }
    }

}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {

    fn next(&self) -> Self {

        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }

    }

}

#[derive(Debug, PartialEq)]
enum MoveResult {
    Inside,
    OutOfBounds
}

const CHAR_UP: char = '^';
const CHAR_DOWN: char = 'v';
const CHAR_LEFT: char = '<';
const CHAR_RIGHT: char = '>';

const CHAR_OBSTACLE: char = '#';
const CHAR_CLEAR: char = '.';
const CHAR_PASSED: char = 'X';

impl Direction {
    pub fn character(&self) -> char {
        match self {
            Direction::Up => CHAR_UP,
            Direction::Down => CHAR_DOWN,
            Direction::Left => CHAR_LEFT,
            Direction::Right => CHAR_RIGHT,
        }
    }
}


fn find_starting_position(matrix: &Vec<Vec<char>>) -> (Position, Direction) {
    for (y, row) in matrix.iter().enumerate() {

        for (x, character) in row.iter().enumerate() {

            let position = Position {
                x: x as i64,
                y: y as i64
            };

            let c = *character;
            return match c {
                CHAR_DOWN => (position, Direction::Down),
                CHAR_UP => (position, Direction::Up),
                CHAR_LEFT => (position, Direction::Left),
                CHAR_RIGHT => (position, Direction::Right),
                _ => {
                    continue
                }
            }
        
        }
        
    }

    panic!("Starting position not found")
}

fn print_debug(matrix: &Vec<Vec<char>>) {

    let columns: usize = matrix.len();
    let rows: usize = matrix[0].len();

    let mut buffer = "".to_string();
    for column in 0..columns {
        for row in 0..rows {
        
            buffer.push('\t');
            buffer.push(matrix[column][row]);
            
        }
        buffer.push('\n');
    }

    println!("{buffer}");

}

fn apply_movement(matrix: &mut Vec<Vec<char>>, position: &mut Position, direction: &mut Direction) -> MoveResult {

    matrix[position.y as usize][position.x as usize] = CHAR_CLEAR;
    let new_position: Position;
    
    loop {
        let updated_position = position.next(&direction);

        if updated_position.x < 0 || updated_position.y < 0 {
            return MoveResult::OutOfBounds;
        } else if updated_position.x >= matrix.len() as i64 || updated_position.y >= matrix[0].len() as i64 {
            return MoveResult::OutOfBounds;
        }

        if matrix[updated_position.y as usize][updated_position.x as usize] == CHAR_OBSTACLE {
            *direction = direction.next();
        } else {
            new_position = updated_position;
            break;
        }
    }
    
    matrix[new_position.y as usize][new_position.x as usize] = direction.character();
    *position = new_position;

    MoveResult::Inside
}

fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //part1::run(&contents);
    part2::run(&contents);
}

mod part1 {
    use super::*;

    pub fn run(contents: &str) {

        let mut matrix: Vec<Vec<char>> = contents.split_whitespace()
                                        .map(|it| { 
                                            it.chars().collect()
                                         })
                                        .collect();

        // println!("Starting matrix:");
        // super::print_debug(&matrix);

        let (mut position, mut direction) = find_starting_position(&matrix);
        //println!("Start: {position:?} going {direction:?}");

        let mut visited_positions: HashSet<Position> = HashSet::new();
        visited_positions.insert(position.clone());
        
        loop {
            matrix[position.y as usize][position.x as usize] = CHAR_PASSED;
            let res = apply_movement(&mut matrix, &mut position, &mut direction);
            
            _ = visited_positions.insert(position.clone());

            // println!("After movement:");
            // super::print_debug(&matrix);
            println!("");

            if res == MoveResult::OutOfBounds {
                break;
            }
        }

        let counter = visited_positions.len();
        println!("Part 1 result: {counter}");

    }

    


}

mod part2 {

    use super::*;

    pub fn run(contents: &str) {

        let mut matrix: Vec<Vec<char>> = contents.split_whitespace()
                                        .map(|it| { 
                                            it.chars().collect()
                                         })
                                        .collect();

        // println!("Starting matrix:");
        // super::print_debug(&matrix);

        let (mut position, mut direction) = find_starting_position(&matrix);
        //println!("Start: {position:?} going {direction:?}");

        let mut loop_positions: HashSet<Position> = HashSet::new();

        for (y, row) in matrix.iter().enumerate() {
            for (x, character) in row.iter().enumerate() {

                if *character != CHAR_CLEAR {
                    continue;
                }

                if y as i64 == position.y && x as i64 == position.x {
                    continue
                }

                let mut copy = matrix.clone();
                copy[y][x] = CHAR_OBSTACLE;
                
                if would_loop(&copy, &position, &direction) {
                    let blocker_pos = Position {
                        x: x as i64,
                        y: y as i64
                    };
                    loop_positions.insert(blocker_pos);
                }
            }
        }
        
        let counter = loop_positions.len();
        println!("Part 2 result: {counter}");
        
    }

    fn would_loop(matrix: &Vec<Vec<char>>, position: &Position, direction: &Direction) -> bool {
        
        let mut matrix = matrix.clone();
        let mut position = position.clone();
        let mut direction = direction.clone();
        
        let mut loop_positions: HashSet<(Position, Direction)> = HashSet::new();

        loop {
            matrix[position.y as usize][position.x as usize] = CHAR_CLEAR;
            let res = apply_movement(&mut matrix, &mut position, &mut direction);

            if res == MoveResult::OutOfBounds {
                return false;
            }

            // println!("Testing movement:");
            // super::print_debug(&matrix);
            // println!("");

            let pair = (position.clone(), direction.clone());

            if loop_positions.contains(&pair) {
                return true;
            }

            loop_positions.insert(pair.clone());
        }
    }

}