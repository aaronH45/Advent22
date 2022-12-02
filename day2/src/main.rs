use std::env;
use std::fs;

fn get_lookup_value(x: &str) -> u16 {
    return match  x {
        "A X" => 1 + 3, // Rock Rock
        "A Y" => 2 + 6, // Rock Paper
        "A Z" => 3 + 0, // Rock Scissors

        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,

        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,

        _ => 0,
    };
}

fn get_lookup_value2(x: &str) -> u16 {
    return match  x {
        "A X" => 3 + 0, // Rock lose - Play paper
        "A Y" => 1 + 3, // Rock draw - Rock
        "A Z" => 2 + 6, // Rock win - Paper

        "B X" => 1 + 0, //paper lose - rock
        "B Y" => 2 + 3, //paper tie -paper
        "B Z" => 3 + 6, // paper win - scissors

        "C X" => 2 + 0, // scissors lose - paper
        "C Y" => 3 + 3, // scissors tie - scissors
        "C Z" => 1 + 6, // scissors win - rock

        _ => 0,
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let day = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let v: Vec<&str> = contents.split("\n").collect();

    let mut sum: u16 = 0;

    for i in &v{

        if day == "1" {
            sum += get_lookup_value(&i.trim());
        } else {
            sum += get_lookup_value2(&i.trim());
        }
    }

    println!("{}", sum);

}
