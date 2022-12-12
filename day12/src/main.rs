use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut sum: u32 = 0;

    if part == "1" {
        let mut heights: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

        let mut dp_table: Vec<Vec<usize>> = vec![vec![usize::MAX; heights[0].len()]; heights.len()];

        let rows: usize = dp_table.len();
        let cols: usize = dp_table[0].len();

        let mut S_x = 0;
        let mut S_y = 0;

        'outer: for row in 0..rows {
            for col in 0..cols {
                if heights[row][col] == 'S' {
                    S_x = col;
                    S_y = row;
                    break 'outer;
                }
            }
        }

        'outer: for row in 0..rows {
            for col in 0..cols {
                if heights[row][col] == 'E' {
                    dp_table[row][col] = 0;
                    heights[S_y][S_x] = 'a';
                    heights[row][col] = 'z';
                    for i in 0..100 {
                        dp_table = update_adj(col, row, heights.clone(), dp_table);
                    }
                    break 'outer;
                }
            }
        }

        'outer: for row in 0..rows {
            for col in 0..cols {
                if heights[row][col] == 'a' {
                    println!("{}", dp_table[row][col]);
                }
            }
        }

        // for row in dp_table.clone() {
        //     println!("{:?}", row);
        // }

        // println!("{}", dp_table[S_y][S_x]);
    } else {
        sum = 0;
    }

    // println!("{}", sum);
}

fn update_adj(
    x: usize,
    y: usize,
    heights: Vec<Vec<char>>,
    dp_table: Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let x_offsets: Vec<i16> = vec![-1, 0, 1, 0];
    let y_offsets: Vec<i16> = vec![0, 1, 0, -1];

    let mut new_dp_table = dp_table.clone();

    let mut next_x = usize::MAX;
    let mut next_y = usize::MAX;

    for i in 0..4 {
        let mut min_steps = usize::MAX;

        let x_offset: i16 = x_offsets[i];
        let y_offset: i16 = y_offsets[i];

        let check_x = x as i16 + x_offset;
        let check_y = y as i16 + y_offset;

        let mut new_x = x;
        let mut new_y = y;
        // println!("{} {}", x, y);
        if check_x < 0 || check_y < 0 {
            continue;
        } else {
            new_x = check_x as usize;
            new_y = check_y as usize;
        }

        if new_y >= dp_table.len() || new_x >= dp_table[0].len() {
            continue;
        } else if heights[new_y][new_x] as usize >= heights[y][x] as usize - 1 {
            if (dp_table[y][x] != usize::MAX) && (dp_table[y][x] + 1 < min_steps) {
                min_steps = dp_table[y][x] + 1;
            }
        }
        if min_steps != usize::MAX && min_steps < dp_table[new_y][new_x] {
            new_dp_table[new_y][new_x] = min_steps;
            // add x,y to update
            new_dp_table = update_adj(new_x, new_y, heights.clone(), new_dp_table);
            // next_x = new_x;
            // next_y = new_y;
        }
    }

    // update next in (xy) list
    // for line in new_dp_table.clone() {
    //     println!("{:?}", line);
    // }
    // println!("\n");
    // if next_x != usize::MAX && next_y != usize::MAX {
    //     new_dp_table = update_adj(next_x, next_y, heights, new_dp_table);
    // }

    return new_dp_table;
}

// fn herlper(c: char) -> u32 {

// }
