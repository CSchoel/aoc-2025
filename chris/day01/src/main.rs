use std::env;
use std::fmt::Error;
use std::fs;
use std::path::Path;
use std::string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: &Path = Path::new("input.txt");
    println!("{}", input_path.display());
    let contents = fs::read_to_string(input_path).expect("Test");
    for l in contents.lines() {
        let mut chars = l.trim().chars();
        let direction = match chars.next() {
            Some(c) => c,
            None => continue, // ignore empyt lines
        };
        let number: String = chars.collect();
        println!("{:?} {:?}", direction, number);
    }
}
