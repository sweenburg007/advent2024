use std::fs;

fn main() {
    let file_path = "./input.txt";
    println!("In file {file_path}");

    // read in the data
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // split by whitespace
    let id_strs = contents.split_whitespace();

    // couldn't figure out how to do this in one line, so just doing it the long way
    let mut id_ints: Vec<i32> = Vec::new();
    for whit_str in id_strs {
        let int_id = whit_str.parse::<i32>().unwrap();
        id_ints.push(int_id);
    }

    // split them into the first and second arrays
    let mut first_slice: Vec<i32> = id_ints.iter().step_by(2).cloned().collect();
    let mut second_slice: Vec<i32> = id_ints.iter().skip(1).step_by(2).cloned().collect();

    // sort them both
    first_slice.sort();
    second_slice.sort();

    // get the differences
    let diffs: Vec<i32> = first_slice
        .iter()
        .zip(second_slice.iter())
        .map(|(x, y)| (x - y).abs())
        .collect();

    let sum: i32 = diffs.iter().sum();

    println!("Final dist sum = {}", sum);

    // try and figure out similarity score
    let mut sim_scores: Vec<i32> = Vec::new();

    for first_val in &first_slice {
        let mut cur_score: i32 = 0;

        for second_val in &second_slice {
            if first_val == second_val {
                cur_score += 1;
            }
        }
        sim_scores.push(cur_score);
    }

    let mult_sim_score: Vec<i32> = first_slice
        .iter()
        .zip(sim_scores.iter())
        .map(|(x, y)| x * y)
        .collect();

    let sim_sum: i32 = mult_sim_score.iter().sum();
    println!("Final sim sum = {}", sim_sum);
}
