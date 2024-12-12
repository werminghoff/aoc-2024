fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1::run(&contents);
    part2::run(&contents);
}

mod part1 {

    fn is_double_digit(value: usize) -> bool {
        format!("{value}").len() % 2 == 0
    }

    fn split_digits(value: usize) -> (usize, usize) {        
        let fmt = format!("{value}");
        let res = fmt.split_at(fmt.len() / 2);
        (res.0.parse().unwrap(), res.1.parse().unwrap())
    }

    fn blink(input: &mut Vec<usize>) {

        let mut idx = 0;
        loop {
            if idx >= input.len() {
                break
            }

            let val = input[idx];

            if val == 0 {
                input[idx] = 1
            } else if is_double_digit(val) {
                let new_vals = split_digits(val);
                input[idx] = new_vals.1;
                input.insert(idx, new_vals.0);
                idx +=1;
            } else {
                input[idx] = val * 2024;
            }

            idx += 1;
        }

    }

    fn blink_n_times(input: &mut Vec<usize>, times: usize) {
        if times < 1 {
            return
        }
        for _ in 0..times {
            blink(input);
        }
    }

    pub fn run(contents: &str) {
        let mut input: Vec<usize> = contents.split_whitespace()
                                        .map(|it| it.parse().unwrap())
                                        .collect();

        blink_n_times(&mut input, 25);
        let stones = input.len();
        println!("Part 1 result: {stones}");
    }

}

mod part2 {
    use std::collections::HashMap;

    #[inline]
    fn number_of_digits(value: usize) -> usize {
        (value.ilog10() + 1) as usize
    }

    #[inline]
    fn is_double_digit(value: usize) -> bool {
        //format!("{value}").len() % 2 == 0
        number_of_digits(value) % 2 == 0
    }

    #[inline]
    fn split_digits(value: usize) -> (usize, usize) {        
        let digits = number_of_digits(value);

        let divisor = (10 as usize).pow((digits/2) as u32);

        let left_side = value / divisor;
        let right_side = value - (left_side * divisor);

        (left_side, right_side)
    }

    fn add_cache(cache: &mut HashMap<BlinkerCache, usize>, input: usize, depth: usize, result: usize) {
        let val = BlinkerCache {
            input: input,
            depth: depth
        };
        _ = cache.insert(val, result);
    }

    fn get_cached(cache: &HashMap<BlinkerCache, usize>, input: usize, depth: usize) -> Option<usize> {
        let val = BlinkerCache {
            input: input,
            depth: depth
        };
        match cache.get(&val) {
            Some(v) => Some(*v),
            None => None
        }
    }


    #[inline]
    fn blink_stone_counter(input: usize, cache: &mut HashMap<BlinkerCache, usize>, depth: usize) -> usize {
        let mut wrapper = || {
            if depth == 0 {
                return 1;
            }

            if let Some(cached_val) = get_cached(cache, input, depth) {
                return cached_val;
            }

            if input == 0 {
                return blink_stone_counter(1, cache, depth - 1)
            } else if is_double_digit(input) {
                let new_vals = split_digits(input);
                let left = blink_stone_counter(new_vals.1, cache, depth - 1);
                let right = blink_stone_counter(new_vals.0, cache, depth - 1);
                return left + right;
            } else {
                return blink_stone_counter(input * 2024, cache, depth - 1);
            }
        };

        let res = wrapper();
        add_cache(cache, input, depth, res);
        res
    }

    fn blink_n_times(input: &Vec<usize>, times: usize) {
        if times < 1 {
            return
        }

        let mut cache = HashMap::<BlinkerCache, usize>::new();

        let collected: Vec<usize> = input.iter().map(|it| {
            let res = blink_stone_counter(*it, &mut cache, times);
            return res
        }).collect();

        let mut sum = 0;
        for f in collected {
            sum += f;
        }

        println!("Part 2 result: {sum}");
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct BlinkerCache {
        pub input: usize,
        pub depth: usize
    }

    pub fn run(contents: &str) {
        let input: Vec<usize> = contents.split_whitespace()
                                        .map(|it| it.parse().unwrap())
                                        .collect();

        blink_n_times(&input, 75);
    }

    #[cfg(test)]
    mod tests {
        
        #[test]
        fn test_number_of_digits() {
            assert_eq!(super::number_of_digits(1), 1);
            assert_eq!(super::number_of_digits(10), 2);
            assert_eq!(super::number_of_digits(100), 3);
            assert_eq!(super::number_of_digits(1000), 4);
            assert_eq!(super::number_of_digits(10000), 5);
            assert_eq!(super::number_of_digits(100000), 6);
            assert_eq!(super::number_of_digits(1000000), 7);
            assert_eq!(super::number_of_digits(10000000), 8);
            assert_eq!(super::number_of_digits(100000000), 9);
            assert_eq!(super::number_of_digits(1000000000), 10);
            assert_eq!(super::number_of_digits(10000000000), 11);
            assert_eq!(super::number_of_digits(100000000000), 12);
            assert_eq!(super::number_of_digits(1000000000000), 13);
            assert_eq!(super::number_of_digits(10000000000000), 14);
            assert_eq!(super::number_of_digits(100000000000000), 15);
            assert_eq!(super::number_of_digits(1000000000000000), 16);
            assert_eq!(super::number_of_digits(10000000000000000), 17);
            assert_eq!(super::number_of_digits(100000000000000000), 18);
            assert_eq!(super::number_of_digits(1000000000000000000), 19);
            assert_eq!(super::number_of_digits(10000000000000000000), 20);
        }

        #[test]
        fn test_split() {
            assert_eq!(super::split_digits(10), (1, 0));
            assert_eq!(super::split_digits(1550), (15, 50));
            assert_eq!(super::split_digits(159950), (159, 950));
            assert_eq!(super::split_digits(15900950), (1590, 950));
            assert_eq!(super::split_digits(1590330950), (15903, 30950));
        }

    }
}

