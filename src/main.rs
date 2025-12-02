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
        Ok(_) => println!("opened"),
    }

    let lines : Vec<String> = s.lines().map(|line| line.to_string()).collect();

    let mut pos: i32 = 50;
    for line in lines {
        let (direction,number_str) = line.split_at(1);
        let mut number : i32 = number_str.parse().unwrap();

        if (number > 99) {
            res += number / 100;
            number = number % 100;
        }

        if (direction == "R"){
            pos += number;
            if(pos > 99) {
                res += 1;
                pos -= 100;
            }
        } else {
            pos -= number;
            if (pos < 0){
                res += 1;
                pos += 100;
            }
        }

        if(pos == 0){
            res += 1;
        }
    }



    println!("{}",res);
}
