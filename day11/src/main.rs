use std::env;
use std::fs;


#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<usize>,
    inspection_count: usize,
    equation: Vec<String>,
    test: usize,
    throw_true: usize,
    throw_false: usize,
}

fn build_monkey(items: Vec<usize>, inspection_count: usize, equation: Vec<String>, test: usize, throw_true: usize, throw_false: usize) -> Monkey {
    Monkey {
        items,
        inspection_count,
        equation,
        test,
        throw_true,
        throw_false,
    }
}

impl Monkey {


    pub fn add(&mut self, item: usize) {
        self.items.push(item);
    }

    pub fn inspect_all(&mut self) -> Vec<(usize,usize)>{
        let mut thrown: Vec<(usize,usize)> = Vec::new();
        while self.items.len() > 0 {
            let item = self.items.pop().unwrap();
            thrown.push(self.inspect(item));
        }
        return thrown;
    }

    pub fn inspect(&mut self, item: usize) -> (usize,usize) {
        self.inspection_count += 1;
        // let new_worry = self.evaluate(item) / 3;
        let new_worry = self.evaluate(item) % 9699690;
        // let new_worry = self.evaluate(item);
        if new_worry % self.test == 0{
            return (new_worry, self.throw_true);
        } else {
            return (new_worry, self.throw_false);
        }
    }



    pub fn evaluate(&mut self, item: usize) -> usize {
        let v1: &str = &self.equation[0];
        let mut v1_val: usize = 0;
        let op: &str = &self.equation[1];
        let v2: &str = &self.equation[2];
        let mut v2_val: usize = 0;

        match v1.trim() {
            "old" => {
                v1_val = item;
            }
            _ => {
                v1_val = v1.parse::<usize>().unwrap();
            }
        }
        match v2.trim() {
            "old" => {
                v2_val = item;
            }
            _ => {
                v2_val = v2.parse::<usize>().unwrap();
            }
        }
        // println!("{} {} {}", v1_val, op, v2_val);
        match op.trim() {
            "+" => {
                return v1_val + v2_val;
            }
            "-" => {
                return v1_val - v2_val;
            }
            "*" => {
                // if v1_val == v2_val {
                //     return v1_val;
                // }
                return v1_val * v2_val;
            }
            "/" => {
                return v1_val / v2_val;
            }
            _ => {
                return 0;
            }
        }        
    }

}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    // let lines: Vec<&str> = contents.lines().collect();
    let monkeys_contents: Vec<&str> = contents.split("\r\n\r\n").collect();
    // println!("{:?}", monkeys);
    let mut monkeys: Vec<Monkey> = Vec::new();
    
    let mut sum: usize = 0;


    if part == "1" {



        for m in &monkeys_contents{
            let attributes: Vec<&str> = m.lines().collect();

            let (_txt, s_items) = attributes[1].split_once(":").unwrap();
            let item_list: Vec<usize> = s_items.trim().split(",").map(|x| x.trim().parse::<usize>().unwrap()).collect();
            // println!("{:?}", item_list);


            let (_lhs, rhs) = attributes[2].split_once("=").unwrap();
            let eq: Vec<String> = rhs.trim().split(" ").map(|x| x.to_string()).collect();
            // println!("{:?}", eq);

            let test: usize = attributes[3].split(" ").last().unwrap().parse::<usize>().unwrap();
            // println!("{:?}", test);

            let t_throw = attributes[4].split(" ").last().unwrap().parse::<usize>().unwrap();
            // println!("{:?}", t_throw);

            let f_throw = attributes[5].split(" ").last().unwrap().parse::<usize>().unwrap();
            // println!("{:?}", f_throw);

            
            let monkey = build_monkey(item_list, 0, eq, test, t_throw, f_throw);

            monkeys.push(monkey);

        }


        // for round in 0..20 {
        for _round in 0..10000 {

            let num_monkeys:usize = monkeys.len();
            for i in 0..num_monkeys {
                let curr_monkey = &mut monkeys[i].clone();
                let thrown = curr_monkey.inspect_all();

                for (new_worry, new_m_idx) in thrown {

                    monkeys[new_m_idx as usize].add(new_worry);
                }
                monkeys[i] = curr_monkey.to_owned();


            }






        }

        // println!("{:?}", monkeys);
        let mut inspection_counts: Vec<usize> = monkeys.iter().map(|x| x.inspection_count).collect();
        
        println!("{:?}", inspection_counts);
        inspection_counts.sort();
        inspection_counts.reverse();

        // for monkey in monkeys {

        // }
        sum = inspection_counts[0]*inspection_counts[1];




    } else {
        sum = 0;
    }

    println!("{}", sum);

}


// fn helper2(s1: &str, s2: &str) -> char {

// }


// fn herlper(c: char) -> usize {

// }