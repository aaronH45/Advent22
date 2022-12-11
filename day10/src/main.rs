use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();



    let mut sum: i32 = 0;

    let mut clock: u32 = 0;
    let mut reg: i32 = 1;
    let mut row: String = "".to_string();

    if part == "1" {

        for line in &lines{
            let split_line = line.split(" ").collect::<Vec<&str>>();
            let op = split_line[0];

            match op {
                "noop" => {
                    clock += 1;
                    sum += check_clock(clock, reg);
                    row = update_row(clock, reg, &mut row);
                },
                "addx" => {
                    clock += 1;
                    sum += check_clock(clock, reg);
                    row = update_row(clock, reg, &mut row);



                    clock += 1;
                    sum += check_clock(clock, reg);
                    row = update_row(clock, reg, &mut row);


                    let inc = split_line[1].parse::<i32>().unwrap();
                    reg += inc;
                },
                _ => {panic!("Unknown op: {}", op);}
            }
            // println!("{} {}", clock,reg);
        }




        
    } else {
        sum = 0;
    }

    println!("{}", sum);

}


fn check_clock(clock: u32, reg: i32) -> i32 {
    let clock_i32: i32 = clock as i32;


    match clock {
        20 | 60 | 100 | 140 | 180 | 220 => { 
            // println!("{} {}", clock,reg); 
            return clock_i32*reg;}
        _ => {return 0;}
    }
}


fn update_row(clock: u32, reg: i32, row: &mut String) -> String {

    
    // println!("{} {}", clock, offset);

    let clock_i32: i32 = clock as i32;
    let pixel = clock_i32 - 1;

    let offset: i32 =  clock_i32 /40;
    let val: i32 = reg + offset*40;

    if pixel >= val - 1 && pixel <= val + 1 {
        row.push('#');
    } else {
        row.push('.');
    }
    // println!(" {} {} {}", clock, reg, row);

    match clock {
        40 | 80 | 120 | 160 | 200 | 240 => { 
            println!("{:?}", row); return "".to_string();
        }
        _ => {return row.to_string();}
    }

}