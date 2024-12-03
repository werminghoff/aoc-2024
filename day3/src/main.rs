fn main() {
    let file_path = "input/input.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    run_part1(&contents);
    run_part2(&contents);
}

fn run_part1(contents: &str) {

}

fn run_part2(contents: &str) {
    
}