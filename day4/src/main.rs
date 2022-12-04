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
            let v2: Vec<u32> = i.split(|c| c == ',' || c == '-' || c == '\r')
                .filter(|s| !s.is_empty()).map(|x| x.parse::<u32>()
                .unwrap())
                .collect();

            let add = is_contained(v2);
            // println!("{:?}", add);
            sum += add;

            // println!("{:?}", v2);
            
        }

        
    } else {
        for i in &v{
            let v2: Vec<u32> = i.split(|c| c == ',' || c == '-' || c == '\r')
                .filter(|s| !s.is_empty()).map(|x| x.parse::<u32>()
                .unwrap())
                .collect();

            let add = overlap(v2);
            // println!("{:?}", add);
            sum += add;

            
        }
    }

    println!("{}", sum);

}


fn is_contained(v:Vec<u32>) -> u32 {

    if v[0] >= v[2] && v[1] <= v[3] {
        return 1 ;
    } else if v[2] >= v[0] && v[3] <= v[1] {
        return 1;
    }
    
    return 0 ;

}


fn overlap(v:Vec<u32>) -> u32 {

    if v[1] >= v[2] && v[0] <= v[2] {
        return 1 ;
    } else if v[3] >= v[0] && v[2] <= v[0] {
        return 1;
    }
    
    return 0 ;

}