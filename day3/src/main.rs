use regex::Regex;
use std::fs;

fn main() {
    println!("Hello, day3!");

    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("Bad file path");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut mult_vals = vec![];

    for (_, [v0, v1]) in re.captures_iter(&contents).map(|x| x.extract()) {
        let int0 = v0.parse::<i32>().unwrap();
        let int1 = v1.parse::<i32>().unwrap();
        mult_vals.push(int0 * int1);
    }
    let sum: i32 = mult_vals.iter().sum();
    println!("Part 1 sum = {sum}");

    // trying part 2 idea where I look for everything in one go
    let re2 = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut flag = true;
    let mut enabled_mult_vals: Vec<i32> = vec![];

    for cap in re2.captures_iter(&contents) {
        let cur_str = cap.get(0).map(|x| x.as_str()).unwrap();
        if cur_str == "do()" {
            flag = true;
        } else if cur_str == "don't()" {
            flag = false;
        } else {
            if flag {
                let int0 = cap
                    .get(1)
                    .map(|x| x.as_str())
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                let int1 = cap
                    .get(2)
                    .map(|x| x.as_str())
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                enabled_mult_vals.push(int0 * int1);
            }
        }
    }
    let sum: i32 = enabled_mult_vals.iter().sum();
    println!("Part 2 sum = {}", sum);
}
