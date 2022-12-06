use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    let buffer: Vec<char> = contents.chars().collect();    
    let mut index: usize = 0;


    if part == "1" {

        for i in 0..buffer.len()-4 as usize{
            let c1: Vec<char> = (&buffer[i..(i+4)]).to_vec();
            let test: bool = compare_all(c1);
            if !test {
                index = i + 4;
                break;
            } 
        }     

    } else {

        for i in 0..buffer.len()-4 as usize{
            let c1: Vec<char> = (&buffer[i..(i+14)]).to_vec();
            let test: bool = compare_all(c1);
            if !test {
                index = i + 14;
                break;
            } 
        }     

    }



    println!("{}", index);

}


fn compare_all(v:Vec<char>) -> bool {
    for (i, el1) in v.iter().enumerate() {
        for (j, el2) in v.iter().enumerate() {
            if i != j && el1 == el2 {
                return true;
            }
        }
    }

    return false;
}

