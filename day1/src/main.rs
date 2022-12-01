use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let v: Vec<&str> = contents.split("\n\n").collect();


    let mut top3:Vec<u32> = vec![0; 3];

    for i in &v{

        let v2: Vec<u32> = i.split("\n").filter(|s| !s.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();

        let sum: u32 = v2.iter().sum();

        if sum >= top3[0]{
            if sum >= top3[1]{
                if sum >= top3[2]{
                    top3[0] = top3[1];
                    top3[1] = top3[2];
                    top3[2] = sum;
                    
                } else {
                    top3[0] = top3[1];
                    top3[1] = sum;
                }
            } else {
                top3[0] = sum;
            }
        }
    }

    println!("{:?}", top3);

    let total:u32 = top3.iter().sum();

    println!("{total}");


}
