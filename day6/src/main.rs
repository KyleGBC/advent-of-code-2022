fn characters_to_get_n_distinct(chars: &[char], window_size: usize) -> usize {
    chars[..].windows(window_size).enumerate().find(|(_, w)|  {
        !(1..w.len()).any(|i| w[i..].contains(&w[i - 1]))
    }).expect("No unique set found").0 + window_size
}
fn main()    {
    let time = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let chars: Vec<char> = input.chars().collect();
    println!("Part 1: {}, Part 2: {}, in {:#?}", characters_to_get_n_distinct(&chars, 4), characters_to_get_n_distinct(&chars, 14), time.elapsed());
} 
