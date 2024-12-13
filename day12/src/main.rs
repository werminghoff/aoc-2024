use std::collections::{HashSet, HashMap};

fn main() {
    let file_path = "input/input-sample3.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    part2::run(&contents);
}

type Matrix = Vec<Vec<char>>;

trait BorderBitFlag {
    fn border_bitflag(top: bool, bottom: bool, left: bool, right: bool) -> Self;
    fn has_top_border(&self) -> bool;
    fn has_bottom_border(&self) -> bool;
    fn has_left_border(&self) -> bool;
    fn has_right_border(&self) -> bool;
}

const BIT_FLAG_TOP: u8      = 0b0001;
const BIT_FLAG_BOTTOM: u8   = 0b0010;
const BIT_FLAG_LEFT: u8     = 0b0100;
const BIT_FLAG_RIGHT: u8    = 0b1000;

impl BorderBitFlag for u8 {

    fn border_bitflag(top: bool, bottom: bool, left: bool, right: bool) -> Self {
        let mut res = 0;
        if top {
            res = res | BIT_FLAG_TOP;
        }
        if bottom {
            res = res | BIT_FLAG_BOTTOM;
        }
        if left {
            res = res | BIT_FLAG_LEFT;
        }
        if right {
            res = res | BIT_FLAG_RIGHT;
        }
        res
    }
    
    fn has_top_border(&self) -> bool {
        (self & BIT_FLAG_TOP) == BIT_FLAG_TOP
    }
    
    fn has_bottom_border(&self) -> bool {
        (self & BIT_FLAG_BOTTOM) == BIT_FLAG_BOTTOM
    }
    
    fn has_left_border(&self) -> bool {
        (self & BIT_FLAG_LEFT) == BIT_FLAG_LEFT
    }

    fn has_right_border(&self) -> bool {
        (self & BIT_FLAG_RIGHT) == BIT_FLAG_RIGHT
    }
}

fn parse(contents: &str) -> Matrix {
    contents.split_whitespace()
                                        .map(|it| { 
                                            it.chars().collect()
                                         })
                                        .collect()
}

#[derive(Debug)]
struct Plot {
    pub name: char,
    pub positions: HashSet<Position>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn to_top(&self) -> Self {
        Self::new(self.x, self.y-1)
    }

    fn to_bottom(&self) -> Self {
        Self::new(self.x, self.y+1)
    }

    fn to_left(&self) -> Self {
        Self::new(self.x-1, self.y)
    }

    fn to_right(&self) -> Self {
        Self::new(self.x+1, self.y)
    }
}

#[derive(Debug)]
struct Cache<'a> {
    pub matrix: &'a Matrix,
    pub borders: HashMap<Position, u8>
}

impl Cache<'_> {

    fn has_left_border_at(&self, position: &Position) -> bool {
        let Some(val) = self.borders.get(&position) else {
            return false;
        };
        val.has_left_border()
    }

    fn has_right_border_at(&self, position: &Position) -> bool {
        let Some(val) = self.borders.get(&position) else {
            return false;
        };
        val.has_right_border()
    }

    fn has_top_border_at(&self, position: &Position) -> bool {
        let Some(val) = self.borders.get(&position) else {
            return false;
        };
        val.has_top_border()
    }

    fn has_bottom_border_at(&self, position: &Position) -> bool {
        let Some(val) = self.borders.get(&position) else {
            return false;
        };
        val.has_bottom_border()
    }

}

fn print_debug_borders(cache: &Cache) {

    let mut buffer = "".to_string();

    for (y, row) in cache.matrix.iter().enumerate() {
        
        let mut top_line = "".to_string();
        let mut middle_line = "".to_string();
        let mut bottom_line = "".to_string();

        for (x, _) in row.iter().enumerate() {
            let position = Position::new(x as i64, y as i64);

            bottom_line.push('\t');
            top_line.push('\t');
            middle_line.push('\t');

            top_line.push(' ');
            if cache.has_top_border_at(&position) {
                top_line.push('-');
            } else {
                top_line.push(' ');
            }
            top_line.push(' ');

            bottom_line.push(' ');
            if cache.has_bottom_border_at(&position) {
                bottom_line.push('-');
            } else {
                bottom_line.push(' ');
            }
            bottom_line.push(' ');

            if cache.has_left_border_at(&position) {
                middle_line.push('|');
            } else {
                middle_line.push(' ');
            }
            
            middle_line.push(cache.matrix[y][x]);

            if cache.has_right_border_at(&position) {
                middle_line.push('|');
            } else {
                middle_line.push(' ');
            }
        }

        buffer.push_str(&top_line);
        buffer.push('\n');
        buffer.push_str(&middle_line);
        buffer.push('\n');
        buffer.push_str(&bottom_line);
        buffer.push('\n');
    }

    println!("{buffer}");

}

fn print_debug_plot(cache: &Cache, plot: &Plot) {

    let mut buffer = "".to_string();

    for (y, row) in cache.matrix.iter().enumerate() {
        
        let mut top_line = "".to_string();
        let mut middle_line = "".to_string();
        let mut bottom_line = "".to_string();

        for (x, _) in row.iter().enumerate() {
            let position = Position::new(x as i64, y as i64);

            bottom_line.push('\t');
            top_line.push('\t');
            middle_line.push('\t');

            if plot.positions.contains(&position) {
                top_line.push(' ');
                if cache.has_top_border_at(&position) {
                    top_line.push('-');
                } else {
                    top_line.push(' ');
                }
                top_line.push(' ');

                bottom_line.push(' ');
                if cache.has_bottom_border_at(&position) {
                    bottom_line.push('-');
                } else {
                    bottom_line.push(' ');
                }
                bottom_line.push(' ');

                if cache.has_left_border_at(&position) {
                    middle_line.push('|');
                } else {
                    middle_line.push(' ');
                }

                middle_line.push(cache.matrix[y][x]);

                if cache.has_right_border_at(&position) {
                    middle_line.push('|');
                } else {
                    middle_line.push(' ');
                }
            } else {
                top_line.push(' ');
                top_line.push(' ');
                top_line.push(' ');
                
                middle_line.push(' ');
                middle_line.push('.');
                middle_line.push(' ');
                
                bottom_line.push(' ');
                bottom_line.push(' ');
                bottom_line.push(' ');
            }

            
        }

        buffer.push_str(&top_line);
        buffer.push('\n');
        buffer.push_str(&middle_line);
        buffer.push('\n');
        buffer.push_str(&bottom_line);
        buffer.push('\n');
    }

    println!("{buffer}");

}

mod part1 {
    
    use super::Matrix;
    use crate::Cache;
    use super::Position;
    use super::Plot;
    use super::BorderBitFlag;
    use std::collections::{HashMap, HashSet};

    pub fn run(contents: &str) {
        let input = super::parse(contents);
        let mut cache = Cache {
            matrix: &input,
            borders: HashMap::new()
        };

        create_border_cache(&mut cache);
        let plots = find_plots(&mut cache);

    }

    fn create_border_cache(data: &mut Cache) {
        for (y, row) in data.matrix.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {

                let position = Position::new(x as i64, y as i64);

                let left_border = has_left_border(&position, &data.matrix);
                let right_border = has_right_border(&position, &data.matrix);
                let top_border = has_top_border(&position, &data.matrix);
                let bottom_border = has_bottom_border(&position, &data.matrix);

                let val = u8::border_bitflag(top_border, bottom_border, left_border, right_border);
                _ = data.borders.insert(position, val);

            }
        }
    }

    fn find_plots(cache: &mut Cache) -> Vec<Plot> {

        let mut plots: Vec<Plot> = Vec::new();
        let mut visited: HashSet<Position> = HashSet::new();

        for y in 0..cache.matrix.len() {
            for x in 0..cache.matrix[y].len() {
                let position = Position::new(x as i64, y as i64);
                let mut current_plot = Plot {
                    name: cache.matrix[y][x],
                    positions: HashSet::new()
                };
                find_plot_visiting(cache, &mut visited, &mut current_plot, position);
                if current_plot.positions.len() > 0 {
                    super::print_debug_plot(cache, &current_plot);
                    plots.push(current_plot);
                    println!("====================================================================================");
                }
            }
        }
        plots
    }

    fn find_plot_visiting(cache: &mut Cache, visited: &mut HashSet<Position>, current_plot: &mut Plot, position: Position) {
        // not visited yet
        if visited.contains(&position) {
            return
        }
        // within matrix bounds
        if position.x < 0 || position.y < 0 {
            return
        }
        if position.y as usize >= cache.matrix.len() {
            return
        }
        if position.x as usize >= cache.matrix[position.y as usize].len() {
            return
        }
        // is same plot name
        if cache.matrix[position.y as usize][position.x as usize] != current_plot.name {
            return
        }

        _ = current_plot.positions.insert(position.clone());
        _ = visited.insert(position.clone());

        find_plot_visiting(cache, visited, current_plot, position.to_left());
        find_plot_visiting(cache, visited, current_plot, position.to_right());
        find_plot_visiting(cache, visited, current_plot, position.to_top());
        find_plot_visiting(cache, visited, current_plot, position.to_bottom());
    }

    fn has_top_border(position: &Position, matrix: &Matrix) -> bool {
        if position.y < 0 {
            false
        } else if position.y == 0 {
            true
        } else if matrix[position.y as usize][position.x as usize] != matrix[position.y  as usize -1][position.x as usize] {
            true
        } else {
            false
        }
    }

    fn has_bottom_border(position: &Position, matrix: &Matrix) -> bool {
        let limit = matrix.len() - 1;
        if position.y as usize > limit {
            false
        } else if position.y as usize == limit {
            true
        } else if matrix[position.y as usize][position.x as usize] != matrix[position.y as usize + 1][position.x as usize] {
            true
        } else {
            false
        }
    }

    fn has_left_border(position: &Position, matrix: &Matrix) -> bool {
        if position.x < 0 {
            false
        } else if position.x == 0 {
            true
        } else if matrix[position.y as usize][position.x as usize] != matrix[position.y as usize][position.x as usize -1] {
            true
        } else {
            false
        }
    }
    
    fn has_right_border(position: &Position, matrix: &Matrix) -> bool {
        let limit = matrix[position.y as usize].len() - 1;
        if position.x as usize > limit {
            false
        } else if position.x as usize == limit {
            true
        } else if matrix[position.y as usize][position.x as usize] != matrix[position.y as usize][position.x as usize + 1] {
            true
        } else {
            false
        }
    }

}

mod part2 {

    pub fn run(contents: &str) {
    }
}