fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    part2::run(&contents);
}

type Matrix = Vec<Vec<char>>;

fn is_antenna_frequency(character: char) -> bool {
    if character >= 'a' && character <= 'z' {
        return true;
    }

    if character >= 'A' && character <= 'Z' {
        return true;
    }

    if character >= '0' && character <= '9' {
        return true;
    }
    
    false
}

mod part1 {
    use std::collections::{HashMap, HashSet};
    use super::Matrix;

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Position {
        pub x: i64,
        pub y: i64
    }

    impl Position {

        fn new(x: i64, y: i64) -> Self {
            Self {
                x: x,
                y: y
            }
        }

        fn is_inside_bounds(&self, matrix: &Matrix) -> bool {
            if self.x < 0 || self.y < 0 {
                return false;
            } else if self.x >= matrix.len() as i64 || self.y >= matrix[0].len() as i64 {
                return false;
            }

            return true;
        }

        fn antinode_position(&self, other: &Self) -> (Self, Self) {
            let diff_x = self.x - other.x;
            let diff_y = self.y - other.y;
            let a = Self::new(self.x + diff_x, self.y + diff_y);
            let b = Self::new(other.x - diff_x, other.y - diff_y);
            (a, b)
        }

    }

    pub fn run(contents: &str) {

        let input_matrix: Matrix = contents.split_whitespace()
                                        .map(|it| { 
                                            it.chars().collect()
                                         })
                                        .collect();
        
        let all_antennas = gather_antennas(&input_matrix);

        let mut antinodes_set = HashSet::<Position>::new();
        
        for (_, positions) in all_antennas.iter() {

            let pos_count = positions.len();
            for a in 0..pos_count {
                for b in a..pos_count {
                    if a == b {
                        continue;
                    }
                    
                    let pos_a = &positions[a];
                    let pos_b = &positions[b];

                    let (anti_a, anti_b) = pos_a.antinode_position(&pos_b);

                    if anti_a.is_inside_bounds(&input_matrix) {
                        antinodes_set.insert(anti_a);
                    }
                    if anti_b.is_inside_bounds(&input_matrix) {
                        antinodes_set.insert(anti_b);
                    }
                }
            }
        }

        let counter: usize = antinodes_set.len();

        println!("Part 1 result: {counter}");
    }

    fn gather_antennas(matrix: &Matrix) -> HashMap<char, Vec<Position>> {
        let mut res = HashMap::new();

        for (y, row) in matrix.iter().enumerate() {

            for (x, character) in row.iter().enumerate() {

                if super::is_antenna_frequency(*character) {
                    
                    let pos = Position::new(x as i64, y as i64);
                    
                    let mut positions: Vec<Position> = match res.remove(character) {
                        Some(p) => p,
                        None => Vec::new()
                    };
                    positions.push(pos);

                    _ = res.insert(*character, positions);
                }

            }
        }

        res
    }

}

mod part2 {
    use std::collections::{HashMap, HashSet};
    use super::Matrix;

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Position {
        pub x: i64,
        pub y: i64
    }

    impl Position {

        fn new(x: i64, y: i64) -> Self {
            Self {
                x: x,
                y: y
            }
        }

        fn is_inside_bounds(&self, matrix: &Matrix) -> bool {
            if self.x < 0 || self.y < 0 {
                return false;
            } else if self.x >= matrix.len() as i64 || self.y >= matrix[0].len() as i64 {
                return false;
            }

            return true;
        }

        fn antinode_position(&self, other: &Self, matrix: &Matrix) -> HashSet<Self> {
            let diff_x = self.x - other.x;
            let diff_y = self.y - other.y;

            let mut res = HashSet::<Self>::new();
            let mut counter: i64 = 0;
            loop {
                let new_pos = Self::new(self.x + (diff_x * counter), self.y + (diff_y * counter));
                if new_pos.is_inside_bounds(matrix) {
                    res.insert(new_pos);
                    counter += 1;
                } else {
                    break;
                }
            }
            counter = 0;

            loop {
                let new_pos = Self::new(other.x - (diff_x * counter), other.y - (diff_y * counter));
                if new_pos.is_inside_bounds(matrix) {
                    res.insert(new_pos);
                    counter += 1;
                } else {
                    break;
                }
            }
            
            res
        }

    }

    pub fn run(contents: &str) {

        let input_matrix: Matrix = contents.split_whitespace()
                                        .map(|it| { 
                                            it.chars().collect()
                                         })
                                        .collect();
        
        let all_antennas = gather_antennas(&input_matrix);

        let mut antinodes_set = HashSet::<Position>::new();
        
        for (_, positions) in all_antennas.iter() {

            let pos_count = positions.len();
            for a in 0..pos_count {
                for b in a..pos_count {
                    if a == b {
                        continue;
                    }
                    
                    let pos_a = &positions[a];
                    let pos_b = &positions[b];

                    let antis = pos_a.antinode_position(&pos_b, &input_matrix);

                    for pos in antis {
                        antinodes_set.insert(pos);
                    }
                    
                }
            }
        }

        let counter: usize = antinodes_set.len();

        println!("Part 2 result: {counter}");
    }

    fn gather_antennas(matrix: &Matrix) -> HashMap<char, Vec<Position>> {
        let mut res = HashMap::new();

        for (y, row) in matrix.iter().enumerate() {

            for (x, character) in row.iter().enumerate() {

                if super::is_antenna_frequency(*character) {
                    
                    let pos = Position::new(x as i64, y as i64);
                    
                    let mut positions: Vec<Position> = match res.remove(character) {
                        Some(p) => p,
                        None => Vec::new()
                    };
                    positions.push(pos);

                    _ = res.insert(*character, positions);
                }

            }
        }

        res
    }

}