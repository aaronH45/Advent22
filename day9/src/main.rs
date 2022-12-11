use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let v: Vec<&str> = contents.lines().collect();

    let mut visited: HashSet<(i16,i16)>  = HashSet::new();
    let mut sum: u32 = 0;

    let mut head_x: i16 = 0;
    let mut head_y: i16 = 0;

    let mut tail_x: i16 = 0;
    let mut tail_y: i16 = 0;

    visited.insert((tail_x,tail_y));

    let mut knots: Vec<Vec<i16>> = vec![vec![0;2]; 9];



    if part == "1" {

        for line in &v{
            let movement = line.split(" ").collect::<Vec<&str>>();
            let direction = movement[0];
            let distance = movement[1].parse::<u32>().unwrap();

            // println!("direction: {} distance: {}", direction, distance);

            for _i in 0..distance {
                match direction {
                    "R" => head_x += 1,
                    "L" => head_x -= 1,
                    "U" => head_y += 1,
                    "D" => head_y -= 1,
                    _ => println!("Error"),
                }
                // println!("head_x: {} head_y: {}", head_x, head_y);
                
                let x_diff: i16 = head_x - tail_x;
                let y_diff: i16 = head_y - tail_y;

                if (x_diff.abs() > 1 && y_diff.abs() == 0) || (y_diff.abs() > 1 && x_diff.abs() == 0) {
                    match direction {
                        "R" => tail_x += 1,
                        "L" => tail_x -= 1,
                        "U" => tail_y += 1,
                        "D" => tail_y -= 1,
                        _ => println!("Error"),
                    }
                } else if x_diff.abs() > 1 && y_diff.abs() == 1 {
                    tail_y = head_y;

                    match direction {
                        "R" => tail_x += 1,
                        "L" => tail_x -= 1,
                        _ => println!("Error"),
                    }

                } else if x_diff.abs() == 1 && y_diff.abs() > 1 {
                    tail_x = head_x;

                    match direction {
                        "U" => tail_y += 1,
                        "D" => tail_y -= 1,
                        _ => println!("Error"),
                    }
                }

                // if x_diff.abs() > 1 || y_diff.abs() > 1 {
                //     // println!("move tail");
                    

                // } 
                visited.insert((tail_x,tail_y));
                // println!("tail_x: {} tail_y: {}", tail_x, tail_y);

            }
        }

        
    } else {


        for line in &v{
            let movement = line.split(" ").collect::<Vec<&str>>();
            let direction = movement[0];
            let distance = movement[1].parse::<u32>().unwrap();


            for _i in 0..distance {
                match direction {
                    "R" => head_x += 1,
                    "L" => head_x -= 1,
                    "U" => head_y += 1,
                    "D" => head_y -= 1,
                    _ => println!("Error"),
                }
                // println!("{} {} head_x: {} head_y: {}", direction, distance, head_x, head_y);
                


                let mut prev_knot = vec![head_x, head_y];
                let mut c = 0;
                for j in 0..knots.len() {
                    let knot = &mut knots[j];
                    // println!("knot: {:?}", knot);

                    let x_diff: i16 = prev_knot[0] - knot[0];
                    let y_diff: i16 = prev_knot[1] -  knot[1];

                    // knot[1] = 1;
                    // println!("x_diff: {} y_diff: {}", x_diff, y_diff);
                    if x_diff.abs() > 1 && y_diff.abs() == 0 {
                        if x_diff > 0 {
                            knot[0] += 1;
                        } else {
                            knot[0] -= 1;
                        }

                    } else if y_diff.abs() > 1 && x_diff.abs() == 0 {

                        if y_diff > 0 {
                            knot[1] += 1;
                        } else {
                            knot[1] -= 1;
                        }
                    


                        // match direction {
                        //     "R" => knot[0] += 1,
                        //     "L" => knot[0] -= 1,
                        //     "U" => knot[1] += 1,
                        //     "D" => knot[1] -= 1,
                        //     _ => println!("Error"),
                        // }
                    
                    } else if x_diff.abs() > 1 && y_diff.abs() == 1 {
                        knot[1] = prev_knot[1];

                        if x_diff > 0 {
                            knot[0] += 1;
                        } else {
                            knot[0] -= 1;
                        }
                        // match direction {
                        //     "R" => knot[0] += 1,
                        //     "L" => knot[0] -= 1,
                        //     _ => println!("Error"),
                        // }

                    } else if x_diff.abs() == 1 && y_diff.abs() > 1 {
                        knot[0] = prev_knot[0];

                        if y_diff > 0 {
                            knot[1] += 1;
                        } else {
                            knot[1] -= 1;
                        }
                        // match direction {
                        //     "U" => knot[1] += 1,
                        //     "D" => knot[1] -= 1,
                        //     _ => println!("Error"),
                        // }
                    } else if x_diff.abs() == 2 && y_diff.abs() == 2 {
                        knot[0] = (knot[0] + prev_knot[0]) /2;
                        knot[1] = (knot[1] + prev_knot[1]) / 2;
                    }

                    c += 1;
                    // println!("knot {}: {} {}",c,  knot[0], knot[1]);
                    prev_knot = knot.to_vec();
                }
                // println!("\n");

                tail_x = knots.last().unwrap()[0];
                tail_y = knots.last().unwrap()[1];
                // println!("{} tail_x: {} tail_y: {}",_i,  tail_x, tail_y);

                visited.insert((tail_x,tail_y));


                // println!("board here");
                // print_board(direction, distance, head_x,head_y, knots.clone());

            }
        }










        
    }


    // println!("visited: {:?}", visited);
    sum = visited.len() as u32;
    println!("{}", sum);

}


// fn helper2(s1: &str, s2: &str) -> char {

// }


fn print_board(direction: &str, distance: u32, head_x: i16, head_y: i16, knots: Vec<Vec<i16>>) {
    println!("direction: {} distance: {}", direction, distance);
    for i in (-10..10).rev() {
        for j in -10..10 {
            if i == 0 && j == 0 {
                print!("s");
            } else if head_x == j && head_y == i {
                print!("H");
            } else if knots.contains(&vec![j,i]) {
                print!("K");
            } else {
                print!(".");
            }
        }
        println!("\n");
    }

    println!("\n");



}