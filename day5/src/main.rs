use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");


    let (boxes, directions) = if let Some((boxes, directions)) 
                = contents.split_once("\r\n\r\n") { (boxes, directions) }
                 else { todo!() };


    if part == "1" {

        let split_boxes: Vec<&str> = boxes.split("\n").collect();

        let mut lines = split_boxes.iter().rev();

        let cols = lines.next().unwrap()
                        .trim().chars()
                        .last().unwrap()
                        .to_digit(10).unwrap();

        // println!("{}", cols );

        let mut vector_boxes: Vec<String> = Vec::new();

        for _col in 0..cols {
            let stack = "".to_owned();
            vector_boxes.push(stack);
        }

        for line in lines {
            let mut chars = line.to_string();

            for i in 0..cols {
                chars.remove((3*i + 3).try_into().unwrap());
            }

            // println!("{}", line);
            let mut new_line =  chars.replace("   ", "[0]");
            new_line.remove(0);
            new_line.pop();
            let row_of_boxes: Vec<&str> = new_line.split("][").collect();  

            // println!("{:?}", row_of_boxes );

            for i in 0..cols as usize {
                // let j = i as usize
                if row_of_boxes[i] != "0"{
                    vector_boxes[i] = vector_boxes[i].to_string() + row_of_boxes[i];
                }

            }
            
        }





        
        let split_directions: Vec<&str> = directions.split("\n").collect();

        for direction in split_directions {
            let dir_nums: Vec<&str> = direction.split(|c| c == ' ' || c == '\r').collect();
            let nums: Vec<usize> = dir_nums.iter().skip(1).step_by(2).copied().map(|x| x.parse::<usize>()
            .unwrap()).collect();

            for _i in 0..nums[0] {
                
                let move_box = vector_boxes[nums[1]-1].pop().unwrap();
                vector_boxes[nums[2]-1].push(move_box);

                // println!("{:?}", vector_boxes);
            }

        }

        println!("{:?}", vector_boxes);
        let mut out = "".to_string();
        for i in vector_boxes{

            out.push(i.chars().last().unwrap());
        }
        println!("{}", out);
        
    } else {

        let split_boxes: Vec<&str> = boxes.split("\n").collect();

        let mut lines = split_boxes.iter().rev();

        let cols = lines.next().unwrap()
                        .trim().chars()
                        .last().unwrap()
                        .to_digit(10).unwrap();

        // println!("{}", cols );

        let mut vector_boxes: Vec<String> = Vec::new();

        for _col in 0..cols {
            let stack = "".to_owned();
            vector_boxes.push(stack);
        }

        for line in lines {
            let mut chars = line.to_string();

            for i in 0..cols {
                chars.remove((3*i + 3).try_into().unwrap());
            }

            // println!("{}", line);
            let mut new_line =  chars.replace("   ", "[0]");
            new_line.remove(0);
            new_line.pop();
            let row_of_boxes: Vec<&str> = new_line.split("][").collect();  

            // println!("{:?}", row_of_boxes );

            for i in 0..cols as usize {
                // let j = i as usize
                if row_of_boxes[i] != "0"{
                    vector_boxes[i] = vector_boxes[i].to_string() + row_of_boxes[i];
                }

            }
            
        }





        
        let split_directions: Vec<&str> = directions.split("\n").collect();

        for direction in split_directions {
            let dir_nums: Vec<&str> = direction.split(|c| c == ' ' || c == '\r').collect();
            let nums: Vec<usize> = dir_nums.iter().skip(1).step_by(2).copied().map(|x| x.parse::<usize>()
            .unwrap()).collect();

            let mut move_box = "".to_string();

            for _i in 0..nums[0] {
                
                let b = vector_boxes[nums[1]-1].pop().unwrap();
                move_box.push(b);

            }

            vector_boxes[nums[2]-1].push_str(&move_box.chars().rev().collect::<String>());

            // println!("{:?}", vector_boxes);

        }

        println!("{:?}", vector_boxes);
        
        let mut out = "".to_string();
        for i in vector_boxes{

            out.push(i.chars().last().unwrap());
        }
        println!("{}", out);

    }

    

}


// fn helper2(s1: &str, s2: &str) -> char {

// }


// fn herlper(c: char) -> u32 {

// }