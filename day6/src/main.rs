fn characters_to_get_n_distinct(chars: &Vec<char>, window_size: usize) -> usize {
    &chars[..].windows(window_size).enumerate().find(|(_, w)|  {
        !(1..w.len()).any(|i| w[i..].contains(&w[i - 1]))
    }).expect("No unique set found").0 + window_size
}
fn main()    {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read in file");
    let chars: Vec<char> = input.chars().collect();
    println!("Part 1: {}, Part 2: {}", characters_to_get_n_distinct(&chars, 4), characters_to_get_n_distinct(&chars, 14));
} 