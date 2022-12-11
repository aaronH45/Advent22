use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let v: Vec<&str> = contents.lines().collect();


    let mut sum: u32 = 0;


    if part == "1" {

        for i in &v{

        }

        
    } else {
        sum = 0;
    }

    println!("{}", sum);

}


// fn helper2(s1: &str, s2: &str) -> char {

// }


// fn herlper(c: char) -> u32 {

// }