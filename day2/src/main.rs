use std::fs;

fn main() {
    let file_path = "./input.txt";
    println!("In file {file_path}");

    // read in the data
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // split the strings by new lines so we can loop over them
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut num_safe_reports = 0;

    for line in lines {
        let report_strs: Vec<&str> = line.split_whitespace().collect();
        let report_ints: Vec<i32> = report_strs
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let report_diffs: Vec<i32> = report_ints.windows(2).map(|x| x[1] - x[0]).collect();

        if report_diffs.iter().all(|&x| x > 0) || report_diffs.iter().all(|&x| x < 0) {
            if !report_diffs.iter().any(|&x| x.abs() > 3 || x.abs() < 1) && report_diffs.len() > 0{
                num_safe_reports += 1;
            }
        }
    }

    println!("Part 1 Number of safe reports = {num_safe_reports}");
}
