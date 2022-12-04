use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let v: Vec<&str> = contents.split("\n").collect();
    let mut sum: u32 = 0;


    if part == "1" {

        for i in &v{
            let len = i.trim().chars().count();
            let half = len/2;

            let firsthalf: &str = &i[0..half];
            let secondhalf: &str = &i[half..len];

            // println!("{} {}", firsthalf, secondhalf);
            let common_char = char_match(firsthalf, secondhalf);

            let value: u32 = unicode_to_value(common_char);
            sum += value;

        }

        
    } else {

        for x in (0..v.len()).step_by(3) {
        
            let s1 = v[x];
            let s2 = v[x+1];
            let s3 = v[x+2];

            let mut matching = String::new();

            for c1 in s1.chars(){
                for c2 in s2.chars() {
                    if c1 == c2 {
                        matching.push(c1);
                    }
                }
            }

            let common_char = char_match(&matching, s3);
    
            let value: u32 = unicode_to_value(common_char);
            sum += value;
            
        }


        
    }

    println!("{}", sum);

}


fn char_match(s1: &str, s2: &str) -> char {
    for c1 in s1.chars(){
        for c2 in s2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }
    return ' ';
}


fn unicode_to_value(c: char) -> u32 {
    let value: u32;
    let unicode = c as u32;

    if unicode > 97 {
        value = unicode - 97 + 1; //lowercase
    } else {
        value = unicode - 65 + 27; //uppercase
    }

    return value;
}