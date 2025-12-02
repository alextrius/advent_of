use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Calculating...,");
    let mut res = 0;
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    let lines : Vec<String> = s.lines().map(|line| line.to_string()).collect();

    for line in lines {
        println!("{}",line);
    }
}
