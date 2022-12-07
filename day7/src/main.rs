use std::env;
use std::fs;
use std::collections::HashMap;

// #[derive(Debug)]
// struct Dir {
//     sub_files: String,
//     size: u32,
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let _part = &args[2];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let v: Vec<&str> = contents.lines().collect();


    let mut sum: u32 = 0;


    // if part == "1" {

        let mut key: String = "".to_string();
        // let mut curr_dir: String = "".to_string();
        // let mut contents: (String, u32) = ("".to_string(), 0);


        // let mut files: HashMap<String, &mut Dir> = HashMap::new();
        let mut files: HashMap<String, u32> = HashMap::new();

        for i in &v{
            let cmd = i[0..4].to_string();
            if cmd == "$ cd" {
                let target = i[5..i.len()].to_string();

                if target == ".." {
                    // key = key[0..key.len()-2].to_string();
                    let key_clone: String = key.clone();
                    let mut split_dirs: Vec<&str> = key_clone.split_whitespace().collect();
                    split_dirs.pop();
                    key = " ".to_string() + &split_dirs.join(" ");


                } else {
                    key = key + " " + &target;
                }


                if !files.contains_key(&key) {
                    files.insert(key.clone(), 0);
                }
            
                
            } else if cmd == "$ ls" {

            } else {
                if cmd == "dir " {
                    // let dir = i[4..i.len()].to_string();
                } else {
                    

                    let file_size: u32 = i.split_whitespace().next().unwrap().parse().unwrap();
                    // let curr_dir: &str = key.split_whitespace().last().unwrap();
                    // println!("{:?}", file_size);
                    // files.get(curr_dir).unwrap() += file_size;

                    // *files.get_mut(&key).unwrap() += file_size;

                    let mut curr_dir: String = key.clone();
                    let key_clone: String = key.clone();
                    let mut split_dirs: Vec<&str> = key_clone.split_whitespace().collect();

                    // let new_dirs = &split_dirs[0..split_dirs.len()-1];
                    // let test: String = " ".to_string() + &new_dirs.join(" ");
                    // println!("{:?} \n", test);
                    // println!("{:?}", split_dirs);


                    while split_dirs.len() > 0 {
                        // println!("{:?}", curr_dir);
                        *files.get_mut(&curr_dir).unwrap() += file_size;

                        // curr_dir = curr_dir[0..curr_dir.len()-2].to_string(); 

                        split_dirs.pop();
                        curr_dir = " ".to_string() + &split_dirs.join(" ");

                        // split_dirs = curr_dir.split_whitespace().collect();
                        // let new_dirs = &split_dirs[0..split_dirs.len()-1];
                        // let test: String = " ".to_string() + &new_dirs.join(" ");
                        // println!("{:?} \n", new_dirs);

                    }

                    // println!(" \n");

                    // println!("{:?}", file_size);
                    // println!("{:?}", curr_dir);
                }
                

                // println!("{:?}", file_size);

            }

            // contents
            // make key value pairs
            // key = ""
            // println!("{:?}", i);
            // println!("{:?} \n", key);

            // println!("{:?} {:?}",i, files);
            // println!("{:?} {:?} ",i, key);


        }

    // println!("{:?}", files);
    

    for (key, value) in files.clone().into_iter() {
        println!("{} {}", key, value);
        // if key == " /" {
        //     space_used = value;
        // }
        if value < 100000 {
            sum += value;
        }
    }

    println!("part one ansswer: {}", sum);


    let space_used: u32 = *files.get(" /").unwrap();

    println!("Space Used: {}", space_used);

    let space_have = 70000000 - space_used;

    println!("Space Needed: {}", space_have);

    let space_needed = 30000000 - space_have;

    println!("Space Needed: {}", space_needed);

    let mut min_size: u32 = 70000000;

    for (key, value) in files.clone().into_iter() {
        // println!("{} {}", key, value);
        if value > space_needed && value < min_size {
            min_size = value;
            println!("{} {}", key, value);
        }
    }

    println!("Smallest directory useable: {}", min_size);
    // } 

    // else {
    //     sum = 0;




        
    // }

    


}


// fn helper2(s1: &str, s2: &str) -> char {

// }


// fn herlper(c: char) -> u32 {

// }