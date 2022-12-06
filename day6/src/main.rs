use std::fs;

fn characters_to_get_n_distinct(chars: &Vec<char>, window_size: usize) -> usize {
    &chars[..].windows(window_size).enumerate().find(|(_, w)|  {
        let mut v = w.to_vec();
        v.sort();
        v.dedup();
        v.len() == window_size
    }).expect("No unique set found").0 + window_size
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read in file");
    let chars: Vec<char> = input.chars().collect();
    println!("Part 1: {}, Part 2: {}", characters_to_get_n_distinct(&chars, 4), characters_to_get_n_distinct(&chars, 14));
} 
