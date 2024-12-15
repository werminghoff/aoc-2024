use std::collections::HashSet;

fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    part2::run(&contents);
}

type Matrix = Vec<Vec<char>>;

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
    pub matrix: &'a Matrix
}

impl Cache<'_> {

    fn has_left_border_at(&self, position: &Position) -> bool {
        if position.x < 0 {
            false
        } else if position.x == 0 {
            true
        } else if self.matrix[position.y as usize][position.x as usize] != self.matrix[position.y as usize][position.x as usize -1] {
            true
        } else {
            false
        }
    }

    fn has_right_border_at(&self, position: &Position) -> bool {
        let limit = self.matrix[position.y as usize].len() - 1;
        if position.x as usize > limit {
            false
        } else if position.x as usize == limit {
            true
        } else if self.matrix[position.y as usize][position.x as usize] != self.matrix[position.y as usize][position.x as usize + 1] {
            true
        } else {
            false
        }
    }

    fn has_top_border_at(&self, position: &Position) -> bool {
        if position.y < 0 {
            false
        } else if position.y == 0 {
            true
        } else if self.matrix[position.y as usize][position.x as usize] != self.matrix[position.y  as usize -1][position.x as usize] {
            true
        } else {
            false
        }
    }

    fn has_bottom_border_at(&self, position: &Position) -> bool {
        let limit = self.matrix.len() - 1;
        if position.y as usize > limit {
            false
        } else if position.y as usize == limit {
            true
        } else if self.matrix[position.y as usize][position.x as usize] != self.matrix[position.y as usize + 1][position.x as usize] {
            true
        } else {
            false
        }
    }    

}

// fn print_debug_borders(cache: &Cache) {

//     let mut buffer = "".to_string();

//     for (y, row) in cache.matrix.iter().enumerate() {
        
//         let mut top_line = "".to_string();
//         let mut middle_line = "".to_string();
//         let mut bottom_line = "".to_string();

//         for (x, _) in row.iter().enumerate() {
//             let position = Position::new(x as i64, y as i64);

//             bottom_line.push('\t');
//             top_line.push('\t');
//             middle_line.push('\t');

//             top_line.push(' ');
//             if cache.has_top_border_at(&position) {
//                 top_line.push('-');
//             } else {
//                 top_line.push(' ');
//             }
//             top_line.push(' ');

//             bottom_line.push(' ');
//             if cache.has_bottom_border_at(&position) {
//                 bottom_line.push('-');
//             } else {
//                 bottom_line.push(' ');
//             }
//             bottom_line.push(' ');

//             if cache.has_left_border_at(&position) {
//                 middle_line.push('|');
//             } else {
//                 middle_line.push(' ');
//             }
            
//             middle_line.push(cache.matrix[y][x]);

//             if cache.has_right_border_at(&position) {
//                 middle_line.push('|');
//             } else {
//                 middle_line.push(' ');
//             }
//         }

//         buffer.push_str(&top_line);
//         buffer.push('\n');
//         buffer.push_str(&middle_line);
//         buffer.push('\n');
//         buffer.push_str(&bottom_line);
//         buffer.push('\n');
//     }

//     println!("{buffer}");

// }

// fn print_debug_plot(cache: &Cache, plot: &Plot) {

//     let mut buffer = "".to_string();

//     for (y, row) in cache.matrix.iter().enumerate() {
        
//         let mut top_line = "".to_string();
//         let mut middle_line = "".to_string();
//         let mut bottom_line = "".to_string();

//         for (x, _) in row.iter().enumerate() {
//             let position = Position::new(x as i64, y as i64);

//             bottom_line.push('\t');
//             top_line.push('\t');
//             middle_line.push('\t');

//             if plot.positions.contains(&position) {
//                 top_line.push(' ');
//                 if cache.has_top_border_at(&position) {
//                     top_line.push('-');
//                 } else {
//                     top_line.push(' ');
//                 }
//                 top_line.push(' ');

//                 bottom_line.push(' ');
//                 if cache.has_bottom_border_at(&position) {
//                     bottom_line.push('-');
//                 } else {
//                     bottom_line.push(' ');
//                 }
//                 bottom_line.push(' ');

//                 if cache.has_left_border_at(&position) {
//                     middle_line.push('|');
//                 } else {
//                     middle_line.push(' ');
//                 }

//                 middle_line.push(cache.matrix[y][x]);

//                 if cache.has_right_border_at(&position) {
//                     middle_line.push('|');
//                 } else {
//                     middle_line.push(' ');
//                 }
//             } else {
//                 top_line.push(' ');
//                 top_line.push(' ');
//                 top_line.push(' ');
                
//                 middle_line.push(' ');
//                 middle_line.push('.');
//                 middle_line.push(' ');
                
//                 bottom_line.push(' ');
//                 bottom_line.push(' ');
//                 bottom_line.push(' ');
//             }

            
//         }

//         buffer.push_str(&top_line);
//         buffer.push('\n');
//         buffer.push_str(&middle_line);
//         buffer.push('\n');
//         buffer.push_str(&bottom_line);
//         buffer.push('\n');
//     }

//     println!("{buffer}");

// }

mod part1 {
    
    use crate::Cache;
    use super::Position;
    use super::Plot;
    use std::collections::HashSet;

    pub fn run(contents: &str) {
        let input = super::parse(contents);
        let mut cache = Cache {
            matrix: &input
        };

        let plots = find_plots(&mut cache);
        let mut result: usize = 0;

        for plot in plots {

            let mut perimeter = 0;
            for pos in &plot.positions {
                if cache.has_bottom_border_at(&pos) {
                    perimeter += 1;
                }
                if cache.has_top_border_at(&pos) {
                    perimeter += 1;
                }
                if cache.has_left_border_at(&pos) {
                    perimeter += 1;
                }
                if cache.has_right_border_at(&pos) {
                    perimeter += 1;
                }
            }
            let area = &plot.positions.len();
            let price = area * perimeter;
            //let name = plot.name;
            //println!("Plot [{name}] area [{area}] perimeter [{perimeter}] price {price}");
            result += price;
        }

        println!("Part 1 result: {result}");

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
                    //super::print_debug_plot(cache, &current_plot);
                    plots.push(current_plot);
                    //println!("====================================================================================");
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

}

mod part2 {

    use crate::Cache;
    use super::Position;
    use super::Plot;
    use std::collections::HashSet;

    pub fn run(contents: &str) {
        let input = super::parse(contents);
        let mut cache = Cache {
            matrix: &input
        };

        let plots = find_plots(&mut cache);
        let mut result: usize = 0;

        for plot in plots {

            let sides = number_of_sides(&plot, &cache);
            let area = &plot.positions.len();
            let price = area * sides;
            //let name = plot.name;
            //println!("Plot [{name}] area [{area}] sides [{sides}] price {price}");
            result += price;
        }

        println!("Part 2 result: {result}");

    }

    fn number_of_sides(plot: &Plot, cache: &Cache) -> usize {
        let mut counter: usize = 0;

        counter += number_of_sides_generic(plot, cache, |it| { cache.has_top_border_at(it) }, |it| { (it.to_left(), it.to_right())});
        counter += number_of_sides_generic(plot, cache, |it| { cache.has_bottom_border_at(it) }, |it| { (it.to_left(), it.to_right())});
        counter += number_of_sides_generic(plot, cache, |it| { cache.has_left_border_at(it) }, |it| { (it.to_top(), it.to_bottom())});
        counter += number_of_sides_generic(plot, cache, |it| { cache.has_right_border_at(it) }, |it| { (it.to_top(), it.to_bottom())});

        counter
    }

    fn next_pos_with_border<'a, FnMatcher>(positions: &'a HashSet<Position>, matcher: &FnMatcher) -> Option<&'a Position> 
    where
        FnMatcher: Fn(&Position) -> bool {
        for pos in positions {
            if matcher(pos) {
                return Some(pos);
            }
        }
        None
    }

    fn next_side<FnMatcher, FnSlider>(positions: &mut HashSet<Position>, cache: &Cache, current_position: &Position, matcher: &FnMatcher, slider: &FnSlider) -> Vec<Position> 
    where
        FnMatcher: Fn(&Position) -> bool,
        FnSlider: Fn(&Position) -> (Position, Position) {

        if !positions.contains(&current_position) {
            return Vec::new();
        }

        let mut all_positions = Vec::new();
                    
        if matcher(current_position) {
            let pos = current_position.clone();
            all_positions.push(current_position.clone());
            positions.remove(&pos);

            let others = slider(current_position);
            
            let mut way = next_side(positions, cache, &others.0, matcher, slider);
            all_positions.append(&mut way);

            let mut way = next_side(positions, cache, &others.1, matcher, slider);
            all_positions.append(&mut way);
        }
        
        all_positions
    }

    fn number_of_sides_generic<FnMatcher, FnSlider>(plot: &Plot, cache: &Cache, matcher: FnMatcher, slider: FnSlider) -> usize 
    where
        FnMatcher: Fn(&Position) -> bool,
        FnSlider: Fn(&Position) -> (Position, Position),
    {
        let mut counter = 0;
        let mut positions = plot.positions.clone();

        loop {
            let Some(pos) = next_pos_with_border(&positions, &matcher) else {
                break
            };

            let current_position = pos.clone();
            let side = next_side(&mut positions, cache, &current_position, &matcher, &slider);
            
            if side.is_empty() {
                break
            }
            
            counter += 1;
        }

        counter
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
                    //super::print_debug_plot(cache, &current_plot);
                    plots.push(current_plot);
                    //println!("====================================================================================");
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
}