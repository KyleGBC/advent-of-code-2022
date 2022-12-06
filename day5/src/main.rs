use regex::Regex;
use std::fs;

fn init_stacks(state: &[&str]) -> [Vec<char>; 9] {
    let mut stacks: [Vec<char>; 9] = Default::default();
    for line in state.iter().rev() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    stacks
}

fn execute_instructions(state: &[&str], instructions: &Vec<(usize, usize, usize)>, reversed: bool) {
    let mut stacks = init_stacks(state);
    for (size, from, to) in instructions {
        let moving_it= stacks[*from].drain(stacks[*from].len() - size..);
        let moving: Vec<char> = if reversed {moving_it.rev().collect()} else {moving_it.collect()};
        stacks[*to].extend(moving);
    }
    stacks.iter().for_each(|s| print!("{}", s.last().unwrap()));
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines: Vec<&str> = input.lines().collect();
    let initial = &lines[..8];
    let mut instructions: Vec<(usize, usize, usize)> = Vec::new();

    let re = Regex::new(r"^move (\d{1,}) from (\d{1,}) to (\d{1,})").expect("Invalid regex");
    for line in lines[10..].iter() {
        let cap = re.captures_iter(line).next().expect("The line did not match the regex");
        let (size, from, to): (usize, usize, usize) = (
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        );
        instructions.push((size, from - 1, to - 1));
    }
    print!("Part 1:\n\t");
    execute_instructions(initial, &instructions, true);
    print!("\nPart 2:\n\t");
    execute_instructions(initial, &instructions, false);
}
