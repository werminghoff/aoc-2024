fn main() {
    let file_path = "input/input-sample.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    //part2::run(&contents);
}

const EMPTY_SPACE: i64 = -1;

mod part1 {
    
    pub fn run(contents: &str) {    
        let mut blocks = load_data(contents);
        defrag(&mut blocks);
        let checksum = calc_checksum(&blocks);
        println!("Part 1 result: {checksum}");
    }

    fn defrag(blocks: &mut Vec<i64>) {
        let mut head: usize = next_empty_index_forward(0, &blocks).unwrap();
        let mut tail = next_nonempty_index_backward(blocks.len() - 1, &blocks).unwrap();

        loop {
            if head >= tail {
                break
            }

            blocks[head] = blocks[tail];
            blocks[tail] = super::EMPTY_SPACE;

            let Some(next_head) = next_empty_index_forward(head, &blocks) else {
                break;
            };
            let Some(next_tail) = next_nonempty_index_backward(tail, &blocks) else {
                break;
            };

            head = next_head;
            tail = next_tail;
        }
    }

    fn calc_checksum(blocks: &Vec<i64>) -> usize {
        let mut sum: usize = 0;

        for (idx, val) in blocks.iter().enumerate() {
            if *val == super::EMPTY_SPACE {
                break;
            }
            sum += idx * (*val as usize);
        }

        sum
    }

    fn next_empty_index_forward(current: usize, blocks: &Vec<i64>) -> Option<usize> {
        let mut idx = current;
        loop {
            let item = &blocks[idx];
            if *item == super::EMPTY_SPACE {
                return Some(idx);
            }
            if idx >= blocks.len() {
                return None;
            }
            idx += 1;
        }
    }

    fn next_nonempty_index_backward(current: usize, blocks: &Vec<i64>) -> Option<usize> {
        let mut idx = current;
        loop {
            let item = &blocks[idx];
            if *item != super::EMPTY_SPACE {
                return Some(idx);
            }
            if idx == 0 {
                return None;
            }
            idx -= 1;
        }
    }

    fn load_data(contents: &str) -> Vec<i64> {
        let mut blocks: Vec<i64> = Vec::new();
        blocks.reserve(contents.len());
        // 23 33 13 31 21 41 41 31 40 2
        
        let mut identifier: i64 = 0;
        let mut chars = contents.chars();

        loop {    
            let Some(file_size) = chars.nth(0) else {
                break;
            };

            let file_size = file_size as usize - '0' as usize;
            for _ in 0..file_size {
                blocks.push(identifier);
            }

            identifier += 1;

            let Some(free_size) = chars.nth(0) else {
                break;
            };
            
            let free_size = free_size as usize - '0' as usize;

            for _ in 0..free_size {
                blocks.push(super::EMPTY_SPACE);
            }
        }
        blocks
    }

}

// mod part2 {

//     struct FileBlock {
//         pub idx: usize,
//         pub size: usize
//     }
    
//     pub fn run(contents: &str) {    
//         let mut blocks = load_data(contents);
//         defrag(&mut blocks);
//         let checksum = calc_checksum(&blocks);
//         println!("Part 1 result: {checksum}");
//     }

//     fn defrag(blocks: &mut Vec<i64>) {
//         let mut head: usize = next_empty_index_forward(0, &blocks).unwrap();
//         let mut file = next_file(blocks.len() - 1, &blocks).unwrap();

//         loop {
//             if head >= file.idx {
//                 break
//             }

//             blocks[head] = blocks[tail];
//             blocks[tail] = super::EMPTY_SPACE;

//             let Some(next_head) = next_empty_index_forward(head, &blocks) else {
//                 break;
//             };
//             let Some(next_tail) = next_nonempty_index_backward(tail, &blocks) else {
//                 break;
//             };

//             head = next_head;
//             tail = next_tail;
//         }
//     }

//     fn calc_checksum(blocks: &Vec<i64>) -> usize {
//         let mut sum: usize = 0;

//         for (idx, val) in blocks.iter().enumerate() {
//             if *val == super::EMPTY_SPACE {
//                 continue;
//             }
//             sum += idx * (*val as usize);
//         }

//         sum
//     }

//     fn next_empty_index_forward(current: usize, blocks: &Vec<i64>) -> Option<usize> {
//         let mut idx = current;
//         loop {
//             let item = &blocks[idx];
//             if *item == super::EMPTY_SPACE {
//                 return Some(idx);
//             }
//             if idx >= blocks.len() {
//                 return None;
//             }
//             idx += 1;
//         }
//     }

//     fn next_file(current: usize, blocks: &Vec<i64>) -> Option<FileBlock> {
//         let mut idx = current;

//         loop {
//             if idx == 0 {
//                 return None;
//             }

//             let item = &blocks[idx];

//             let item_value = *item;
//             let mut item_idx = idx;
//             loop {
//                 if blocks[idx] != item_value {
//                     break;
//                 }
//                 item_idx -= 1;
//             }

//             if *item != super::EMPTY_SPACE {
//                 let file = FileBlock {
//                     idx: item_idx,
//                     size: idx - item_idx
//                 };
//                 return Some(file);
//             }

//             if idx == 0 {
//                 return None;
//             }
//             idx -= 1;
//         }
//     }

//     fn load_data(contents: &str) -> Vec<i64> {
//         let mut blocks: Vec<i64> = Vec::new();
//         blocks.reserve(contents.len());
//         // 23 33 13 31 21 41 41 31 40 2
        
//         let mut identifier: i64 = 0;
//         let mut chars = contents.chars();

//         loop {    
//             let Some(file_size) = chars.nth(0) else {
//                 break;
//             };

//             let file_size = file_size as usize - '0' as usize;
//             for _ in 0..file_size {
//                 blocks.push(identifier);
//             }

//             identifier += 1;

//             let Some(free_size) = chars.nth(0) else {
//                 break;
//             };
            
//             let free_size = free_size as usize - '0' as usize;

//             for _ in 0..free_size {
//                 blocks.push(super::EMPTY_SPACE);
//             }
//         }
//         blocks
//     }

// }
