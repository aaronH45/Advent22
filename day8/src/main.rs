use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let trees: Vec<&str> = contents.lines().collect();

    let rows = trees.len();
    let cols = trees[0].len();

    const RADIX: u32 = 10;
    // println!("rows: {}, cols: {}", rows, cols);

    if part == "1" {
        let mut visable = vec![vec![false; cols]; rows];

        // let i: usign = 0;
        // let j: usign = 0;
    
        
        let mut top_max: Vec<u32> = trees[0].chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
        let mut left_max: Vec<u32> = vec![0; rows];
    
        let mut bot_max: Vec<u32> = trees[rows-1].chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
        let mut right_max: Vec<u32> = vec![0; rows];
    
        for i in 0..rows {
            left_max[i] = trees[i].chars().next().unwrap().to_digit(RADIX).unwrap();
            right_max[i] = trees[i].chars().last().unwrap().to_digit(RADIX).unwrap();
        }
    
    
    
        // println!("top_max: {:?}", top_max);
        // println!("left_max: {:?}", left_max);
        // println!("bot_max: {:?}", bot_max);
        // println!("right_max: {:?}", right_max);
        // 
        
    
        for i in 0..rows {
            for j in 0..cols {
                if i == 0 || j == 0 || i == (rows -1) || j == (cols - 1) {
                    visable[i][j] = true;
                    // visible_counter += 1;
                }
                let tree_val = trees[i].chars().nth(j).unwrap().to_digit(RADIX).unwrap();
                // println!("tree_val: {}", tree_val);
    
                if tree_val > top_max[j] {
                    // println!("changed");
                    top_max[j] =tree_val;
                    visable[i][j] = true;
                    // visible_counter += 1;
                    if tree_val > left_max[i] {
                        left_max[i] = tree_val;
                    }
                }
    
                if tree_val > left_max[i] {
                    left_max[i] = tree_val;
                    visable[i][j] = true;
                    // visible_counter += 1;
                    if tree_val > top_max[j] {
                        top_max[j] = tree_val;
                    }
                }
    
            }
        }
    
        for i in (0..rows).rev() {
            for j in (0..cols).rev() {
                let tree_val = trees[i].chars().nth(j).unwrap().to_digit(RADIX).unwrap();
                // println!("tree_val: {}", tree_val);
    
                if tree_val > bot_max[j] {
                    // println!("changed");
                    bot_max[j] = tree_val;
                    visable[i][j] = true;
                    // visible_counter += 1;
                    if tree_val > right_max[i] {
                        right_max[i] = tree_val;
                    }
                }
    
                if tree_val > right_max[i] {
                    right_max[i] = tree_val;
                    visable[i][j] = true;
                    // visible_counter += 1;
                    if tree_val > bot_max[j] {
                        bot_max[j] = tree_val;
                    }
                }
    
            }
        }
    
    
        let mut visible_counter = 0;
    
        for i in 0..rows {
            for j in 0..cols {
                if visable[i][j] == true {
                    visible_counter += 1;
                }
            }
        }
    
    
    
        println!("visible trees: {}", visible_counter);
    
    } else {
        let mut scenic_score = vec![vec![0; cols]; rows];


        for i in 1..(rows-1) {
            for j in 1..(cols-1) {

                let tree_val = trees[i].chars().nth(j).unwrap().to_digit(RADIX).unwrap();

                let mut l_score = 0;
                let mut r_score = 0;
                let mut t_score = 0;
                let mut b_score = 0;

                for k in (0..i).rev() {
                    let tree_ik = trees[k].chars().nth(j).unwrap().to_digit(RADIX).unwrap();
                    if tree_val > tree_ik {
                        t_score += 1;
                    } else  {
                        t_score += 1;
                        break;
                    }
                }

                for k in (i+1)..rows {
                    let tree_ik = trees[k].chars().nth(j).unwrap().to_digit(RADIX).unwrap();
                    if tree_val > tree_ik {
                        b_score += 1;
                    } else  {
                        b_score += 1;
                        break;
                    }
                }

                for k in (0..j).rev() {
                    let tree_ik = trees[i].chars().nth(k).unwrap().to_digit(RADIX).unwrap();
                    if tree_val > tree_ik {
                        l_score += 1;
                    } else  {
                        l_score += 1;
                        break;
                    }
                }

                for k in (j+1)..cols {
                    let tree_ik = trees[i].chars().nth(k).unwrap().to_digit(RADIX).unwrap();
                    if tree_val > tree_ik {
                        r_score += 1;
                    } else  {
                        r_score += 1;
                        break;
                    }
                }

                // println!("l_score: {} t_score: {} r_score: {} b_score: {}", l_score, t_score, r_score, b_score);
                let score = l_score * r_score * t_score * b_score;
                // println!("i: {} j: {} score: {}", i,j,score);
                scenic_score[i][j] = score;
            
                }
    
            }

            // for i in 0..rows {
            //     println!("{:?}", scenic_score[i]);
            // }
        
            let mut max_score = 0;
    
            for i in 0..rows {
                for j in 0..cols {
                    if scenic_score[i][j] > max_score {
                        max_score = scenic_score[i][j];
                    }
                }
            }

            println!("Highest Scenic Score: {}", max_score);

        
        }






}



    





// fn helper2(s1: &str, s2: &str) -> char {

// }


// fn herlper(c: char) -> u32 {

// }