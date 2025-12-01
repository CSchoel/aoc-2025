use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: &Path = Path::new("input.txt");
    println!("{}", input_path.display());
    let contents = fs::read_to_string(input_path).expect("Test");
    println!("{contents}");
}
