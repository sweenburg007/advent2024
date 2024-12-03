use std::fs;

fn validate_report(report: &Vec<i32>) -> bool {
    let report_diffs: Vec<i32> = report.windows(2).map(|x| x[1] - x[0]).collect();

    if (report_diffs.iter().all(|&x| x > 0) || report_diffs.iter().all(|&x| x < 0))
        && !report_diffs.iter().any(|&x| x.abs() > 3 || x.abs() < 1)
    {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let file_path = "./input.txt";
    // let file_path = "./test_input.txt";
    println!("In file {file_path}");

    // read in the data
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // split the strings by new lines so we can loop over them
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut num_safe_reports = 0;
    let mut unsafe_reports: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let report_strs: Vec<&str> = line.split_whitespace().collect();
        let report_ints: Vec<i32> = report_strs
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if report_ints.len() > 0 {
            if validate_report(&report_ints) {
                num_safe_reports += 1;
            } else {
                unsafe_reports.push(report_ints);
            }
        }
    }
    println!("Part 1 Number of safe reports = {num_safe_reports}");

    // println!("Unsafe reports:");
    // println!("{:?}", unsafe_reports);
    println!("------------------------------");

    // loop over all the unsafe reports and see if removing a single value fixes them
    let mut fixable = 0;
    for unsafe_report in unsafe_reports {
        // println!("{:?}", unsafe_report);
        for i in 0..unsafe_report.len() {
            let mut tmp = unsafe_report.clone();
            tmp.remove(i);

            if validate_report(&tmp) {
                fixable += 1;
                // println!("Can fix this, breaking out for next unfix");
                break;
            }
        }
    }
    println!("num ixable reports = {fixable}");
    println!("Part 2 Valid Reports = {}", num_safe_reports + fixable);
}
