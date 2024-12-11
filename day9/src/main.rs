fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    part2::run(&contents);
}

const EMPTY_SPACE: i64 = -1;

fn load_data(contents: &str) -> Vec<i64> {
    let mut blocks: Vec<i64> = Vec::new();
    blocks.reserve(contents.len());
    
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
            blocks.push(EMPTY_SPACE);
        }
    }
    blocks
}

mod part1 {
    
    pub fn run(contents: &str) {    
        let mut blocks = super::load_data(contents);
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

}

mod part2 {

    #[derive(Debug)]
    struct FileBlock {
        pub idx: usize,
        pub size: usize
    }
    
    pub fn run(contents: &str) {    
        let mut blocks = super::load_data(contents);
        defrag(&mut blocks);
        let checksum = calc_checksum(&blocks);
        println!("Part 2 result: {checksum}");
    }

    fn defrag(blocks: &mut Vec<i64>) {

        let mut tail_idx = blocks.len();
        loop {
            if tail_idx == 0 {
                break;
            }
            let Some(file) = next_file(tail_idx - 1, &blocks) else {
                return;
            };

            tail_idx = file.idx;

            let Some(start_idx) = find_empty_slot(0, tail_idx, blocks, file.size) else {
                continue
            };

            for i in 0..file.size {
                blocks[start_idx + i] = blocks[file.idx + i];
                blocks[file.idx + i] = super::EMPTY_SPACE;
            }
        }
    }

    fn calc_checksum(blocks: &Vec<i64>) -> usize {
        let mut sum: usize = 0;

        for (idx, val) in blocks.iter().enumerate() {
            if *val == super::EMPTY_SPACE {
                continue;
            }
            sum += idx * (*val as usize);
        }

        sum
    }

    fn find_empty_slot(from_idx: usize, max_idx: usize, blocks: &Vec<i64>, fitting_size: usize) -> Option<usize> {
        if fitting_size == 0 {
            panic!("fitting_size should not be zero");
        }
        let mut idx = from_idx;
        let mut block_start = from_idx;

        let mut space_available: usize = 0;
        loop {
            if idx >= max_idx {
                return None;
            }

            let item = &blocks[idx];

            idx += 1;
            
            if *item == super::EMPTY_SPACE {
                space_available += 1;
            } else {
                space_available = 0;
                block_start = idx;
            }

            if space_available >= fitting_size {
                return Some(block_start)
            }
        }
    }

    fn next_file(current: usize, blocks: &Vec<i64>) -> Option<FileBlock> {
        if current == 0 {
            return None;
        }
        
        let mut idx = current;

        loop {
            let item = &blocks[idx];
            
            let item_value = *item;
            let mut item_idx = idx;
            let mut item_size: usize = 1;

            loop {
                if item_idx == 0 {
                    let file = FileBlock {
                        idx: 0,
                        size: item_size
                    };
                    return Some(file);
                }
                if blocks[item_idx] != item_value {
                    break;
                }
                item_size += 1;
                item_idx -= 1;
                
            }

            if *item != super::EMPTY_SPACE {
                let file = FileBlock {
                    idx: item_idx + 1,
                    size: item_size - 1
                };
                return Some(file);
            }

            if idx == 0 {
                return None;
            }
            idx -= 1;
        }
    }

}
