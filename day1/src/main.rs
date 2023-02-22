use std::fs;

fn main() {
    let mut summed_calories: Vec<i32> = Vec::new();
    let calories = fs::read_to_string("input.txt").expect("File not found");
    let mut total = 0;

    for line in calories.split("\r\n") {
        if !line.is_empty() {
            total += line.parse::<i32>().expect("Parsing error");
        }
        else {
            summed_calories.push(total);
            total = 0;
        }
    }

    println!("Max calories from one elf: {}", summed_calories.iter().max().expect("Result list was empty!"));
    summed_calories.sort_unstable();
    summed_calories = summed_calories.into_iter().rev().collect();
    summed_calories.truncate(3);
    println!("Calories from the top 3 elves: {}", summed_calories.iter().sum::<i32>());
}
