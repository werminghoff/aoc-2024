fn main() {
    let file_path = "input/input-sample.txt";
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
            }
        }

        // total += count_horizontal_occurences(&matrix, word);
        // total += count_vertical_occurences(&matrix, word);
        // total += count_diagonal1_occurences(&matrix, word);
        // total += count_diagonal2_occurences(&matrix, word);

        println!("Part 1 result: {counter}");
    }
    
    pub fn run_old(contents: &str) {
        let word: &str = "XMAS";
        let matrix: Vec<Vec<char>> = contents.split_whitespace()
                                        .map(|it| { 
                                            it.chars().collect()
                                         })
                                        .collect();
        
        let mut total = 0;
        total += count_horizontal_occurences(&matrix, word);
        total += count_vertical_occurences(&matrix, word);
        // total += count_diagonal1_occurences(&matrix, word);
        // total += count_diagonal2_occurences(&matrix, word);

        println!("Part 1 result: {total}");
    }

    pub fn check_horizontal_match(row: usize, column: usize, word_len: usize, word_chars: &[char], word_chars_rev: &[char], matrix: &Vec<Vec<char>>) -> bool {

        if column >
    
        let mut possible_match: [char; 4] = ['-','-','-','-'];

        let mut positions = Vec::<(usize,usize)>::new();

        for i in 0..word_len {
            possible_match[i] = matrix[row][column+i];
            positions.push((row, column+i));
        }

        if possible_match == word_chars {
            println!("Matched [horizontal-1] {possible_match:?} at col {column} x row {row}");
            print_debug(matrix, &positions);
            return true;
        } else if possible_match == word_chars_rev {
            println!("Matched [horizontal-2] {possible_match:?} at col {column} x row {row}");
            print_debug(matrix, &positions);
            return true;
        }

        return false;
    }

    pub fn count_horizontal_occurences(matrix: &Vec<Vec<char>>, word: &str) -> usize {
        let mut counter: usize = 0;
        let word_len = word.len();

        let columns: usize = matrix.len();
        let rows: usize = matrix[0].len();

        let word_vec: Vec<char> = word.chars().collect();
        let word_chars: &[char] = &word_vec[0..];
        
        let mut word_vec_rev: Vec<char> = word_vec.clone();
        word_vec_rev.reverse();
        let word_chars_rev: &[char] = &word_vec_rev[0..];

        for col in 0..columns-(word_len-1) {
            for row in 0..rows {
                let mut possible_match: [char; 4] = ['-','-','-','-'];

                let mut positions = Vec::<(usize,usize)>::new();
        
                for i in 0..word_len {
                    possible_match[i] = matrix[row][col+i];
                    positions.push((row, col+i));
                }

                if possible_match == word_chars {
                    println!("Matched [horizontal-1] {possible_match:?} at col {col} x row {row}");
                    print_debug(matrix, &positions);
                    counter += 1;
                } else if possible_match == word_chars_rev {
                    println!("Matched [horizontal-2] {possible_match:?} at col {col} x row {row}");
                    print_debug(matrix, &positions);
                    counter += 1;
                }
            
            }
        }

        counter
    }

    pub fn count_vertical_occurences(matrix: &Vec<Vec<char>>, word: &str) -> usize {
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
            for row in 0..rows-(word_len-1) {
                let mut possible_match: [char; 4] = ['-','-','-','-'];

                let mut positions = Vec::<(usize,usize)>::new();
        
                for i in 0..word_len {
                    possible_match[i] = matrix[row+i][col];
                    positions.push((row+i, col));
                }

                if possible_match == word_chars {
                    println!("Matched [vertical-1] {possible_match:?} at col {col} x row {row}");
                    print_debug(matrix, &positions);
                    counter += 1;
                } else if possible_match == word_chars_rev {
                    println!("Matched [vertical-2] {possible_match:?} at col {col} x row {row}");
                    print_debug(matrix, &positions);
                    counter += 1;
                }
            
            }
        }

        counter
    }

    pub fn count_diagonal1_occurences(matrix: &Vec<Vec<char>>, word: &str) -> usize {
        0
    }

    pub fn count_diagonal2_occurences(matrix: &Vec<Vec<char>>, word: &str) -> usize {
        0
    }
}

mod part2 {
    pub fn run(contents: &str) {

    }
}
