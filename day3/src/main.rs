use regex::Regex;
use std::fs;

fn main() {
    println!("Hello, day3!");

    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    let mut mult_vals = vec![];
    for (_, [v0, v1]) in re.captures_iter(&contents).map(|x| x.extract()) {
        let int0 = v0.parse::<i32>().unwrap();
        let int1 = v1.parse::<i32>().unwrap();

        mult_vals.push(int0 * int1);
    }
    let sum: i32 = mult_vals.iter().sum();
    println!("Sum = {sum}");
}
