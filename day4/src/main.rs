fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    part2::run(&contents);
}

mod part1 {

    fn print_debug(matrix: &Vec<Vec<char>>, keeping: &Vec<(usize, usize)>) {

        let columns: usize = matrix.len();
        let rows: usize = matrix[0].len();

        let mut buffer = "".to_string();
        for column in 0..columns {
            for row in 0..rows {
            
                buffer.push('\t');

                let a = (column, row);
                if keeping.contains(&a) {
                    buffer.push(matrix[column][row]);
                } else {
                    buffer.push('.');
                }

            }
            buffer.push('\n');
        }

        println!("{buffer}");

    }

    pub fn run(contents: &str) {
        let word: &str = "XMAS";
        let matrix: Vec<Vec<char>> = contents.split_whitespace()
                                        .map(|it| { 
                                            it.chars().collect()
                                         })
                                        .collect();

        let mut counter: usize = 0;
        let word_len = word.len();

        let columns: usize = matrix.len();
        let rows: usize = matrix[0].len();

        let word_vec: Vec<char> = word.chars().collect();
        let word_chars: &[char] = &word_vec[0..];
        
        let mut word_vec_rev: Vec<char> = word_vec.clone();
        word_vec_rev.reverse();
        let word_chars_rev: &[char] = &word_vec_rev[0..];
        
        for col in 0..columns {
            for row in 0..rows {
                if check_horizontal_match(row, col, word_len, word_chars, word_chars_rev, &matrix) {
                    counter += 1;
                }
                if check_vertical_match(row, col, word_len, word_chars, word_chars_rev, &matrix) {
                    counter += 1;
                }
                if check_diagonal1_match(row, col, word_len, word_chars, word_chars_rev, &matrix) {
                    counter += 1;
                }
                if check_diagonal2_match(row, col, word_len, word_chars, word_chars_rev, &matrix) {
                    counter += 1;
                }
            }
        }

        println!("Part 1 result: {counter}");
    }
    
    pub fn validate_match(possible_match: &[char], word_chars: &[char], word_chars_rev: &[char]) -> bool {
        if possible_match == word_chars {
            return true;
        } else if possible_match == word_chars_rev {
            return true;
        }
        return false
    }

    pub fn check_vertical_match(row: usize, column: usize, word_len: usize, word_chars: &[char], word_chars_rev: &[char], matrix: &Vec<Vec<char>>) -> bool {

        if row > (matrix[0].len() - word_len) {
            return false;
        }
    
        let mut possible_match: [char; 4] = ['-','-','-','-'];

        let mut positions = Vec::<(usize,usize)>::new();

        for i in 0..word_len {
            possible_match[i] = matrix[row+i][column];
            positions.push((row+i, column));
        }

        let res= validate_match(&possible_match, word_chars, word_chars_rev);
        // if res {
        //     println!("Matched [vertical] {possible_match:?} at col {column} x row {row}");
        //     print_debug(matrix, &positions);
        // }
        res
    }

    pub fn check_horizontal_match(row: usize, column: usize, word_len: usize, word_chars: &[char], word_chars_rev: &[char], matrix: &Vec<Vec<char>>) -> bool {

        if column > (matrix.len() - word_len) {
            return false;
        }
    
        let mut possible_match: [char; 4] = ['-','-','-','-'];

        let mut positions = Vec::<(usize,usize)>::new();

        for i in 0..word_len {
            possible_match[i] = matrix[row][column+i];
            positions.push((row, column+i));
        }

        let res= validate_match(&possible_match, word_chars, word_chars_rev);
        // if res {
        //     println!("Matched [horizontal] {possible_match:?} at col {column} x row {row}");
        //     print_debug(matrix, &positions);
        // }
        res
    }

    // top-left to bottom-right
    pub fn check_diagonal1_match(row: usize, column: usize, word_len: usize, word_chars: &[char], word_chars_rev: &[char], matrix: &Vec<Vec<char>>) -> bool {

        if column > (matrix.len() - word_len) || row > (matrix[0].len() - word_len) {
            return false;
        }
    
        let mut possible_match: [char; 4] = ['-','-','-','-'];

        let mut positions = Vec::<(usize,usize)>::new();

        for i in 0..word_len {
            possible_match[i] = matrix[row+i][column+i];
            positions.push((row+i, column+i));
        }

        let res= validate_match(&possible_match, word_chars, word_chars_rev);
        // if res {
        //     println!("Matched [diagonal1] {possible_match:?} at col {column} x row {row}");
        //     print_debug(matrix, &positions);
        // }
        res
    }

    // bottom-left to top-right
    pub fn check_diagonal2_match(row: usize, column: usize, word_len: usize, word_chars: &[char], word_chars_rev: &[char], matrix: &Vec<Vec<char>>) -> bool {
        if column > (matrix.len() - word_len) || row > (matrix[0].len() - word_len) {
            return false;
        }
    
        let mut possible_match: [char; 4] = ['-','-','-','-'];

        let mut positions = Vec::<(usize,usize)>::new();

        for i in 0..word_len {
            let actual_row = row+(word_len-i-1);
            let actual_col = column+i;
            possible_match[i] = matrix[actual_row][actual_col];
            positions.push((actual_row, actual_col));
        }

        let res= validate_match(&possible_match, word_chars, word_chars_rev);
        // if res {
        //     println!("Matched [diagonal2] {possible_match:?} at col {column} x row {row}");
        //     print_debug(matrix, &positions);
        // }
        res
    }
}

mod part2 {

    fn print_debug(matrix: &Vec<Vec<char>>, keeping: &Vec<(usize, usize)>) {
        let columns: usize = matrix.len();
        let rows: usize = matrix[0].len();

        let mut buffer = "".to_string();
        for column in 0..columns {
            for row in 0..rows {
            
                buffer.push('\t');

                let a = (column, row);
                if keeping.contains(&a) {
                    buffer.push(matrix[column][row]);
                } else {
                    buffer.push('.');
                }

            }
            buffer.push('\n');
        }

        println!("{buffer}");
    }

    pub fn run(contents: &str) {
        let word: &str = "MAS";
        let matrix: Vec<Vec<char>> = contents.split_whitespace()
                                        .map(|it| { 
                                            it.chars().collect()
                                         })
                                        .collect();

        let mut counter: usize = 0;
        let word_len = word.len();

        let columns: usize = matrix.len();
        let rows: usize = matrix[0].len();

        let word_vec: Vec<char> = word.chars().collect();
        let word_chars: &[char] = &word_vec[0..];
        
        let mut word_vec_rev: Vec<char> = word_vec.clone();
        word_vec_rev.reverse();
        let word_chars_rev: &[char] = &word_vec_rev[0..];
        
        for col in 0..columns {
            for row in 0..rows {
                if check_match(row, col, word_len, word_chars, word_chars_rev, &matrix) {
                    counter += 1;
                }
            }
        }

        println!("Part 2 result: {counter}");
    }
    
    pub fn validate_match(possible_match: &[char], word_chars: &[char], word_chars_rev: &[char]) -> bool {
        if possible_match == word_chars {
            return true;
        } else if possible_match == word_chars_rev {
            return true;
        }
        return false
    }

    pub fn check_match(row: usize, column: usize, word_len: usize, word_chars: &[char], word_chars_rev: &[char], matrix: &Vec<Vec<char>>) -> bool {

        if column > (matrix.len() - word_len) || row > (matrix[0].len() - word_len) {
            return false;
        }
    
        let first_matched: bool;
        let second_matched: bool;
        let mut first_positions = Vec::<(usize,usize)>::new();
        let mut second_positions = Vec::<(usize,usize)>::new();

        {
            let mut possible_match: [char; 3] = ['-','-','-'];        

            for i in 0..word_len {
                let actual_row = row+(word_len-i-1);
                let actual_col = column+i;
                possible_match[i] = matrix[actual_row][actual_col];
                first_positions.push((actual_row, actual_col));
            }

            first_matched = validate_match(&possible_match, word_chars, word_chars_rev);
        }

        {
            let mut possible_match: [char; 3] = ['-','-','-'];

            for i in 0..word_len {
                possible_match[i] = matrix[row+i][column+i];
                second_positions.push((row+i, column+i));
            }

            second_matched = validate_match(&possible_match, word_chars, word_chars_rev);
        }
        

        let res = first_matched && second_matched;
        if res {
            let mut merged_positions = Vec::<(usize,usize)>::new();
            merged_positions.append(&mut first_positions);
            merged_positions.append(&mut second_positions);
            //println!("Matched at col {column} x row {row}");
            //print_debug(matrix, &merged_positions);
        }
        res
    }
}