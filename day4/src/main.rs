use std::fs;
use std::time::Instant;
use std::thread::{self, *};

fn contained_sections(lines: &[&str]) -> (u32, u32) {
    let (mut complete_contained, mut partial_contained) = (0_u32, 0_u32);
    for line in lines {
        let nums: Vec<i32> = line.split(['-', ',']).map(|s| s.parse::<i32>().unwrap()).collect();
        let (assign1_b, assign1_e, assign2_b, assign2_e) = match &nums[..] {
            &[a, b, c, d] => (a, b, c, d),
            _ => panic!("A line didn't parse into four numbers")
        };

        if (assign1_b - assign2_b) * (assign1_e - assign2_e) <= 0 {
            complete_contained += 1;
        }
        if assign1_e >= assign2_b && assign2_e >= assign1_b {
            partial_contained += 1;
        }
    }
    (complete_contained, partial_contained)
}

fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("input.txt").expect("File not found");
    let pair_assignments: Vec<&str> = input.lines().collect();
    let num_threads: usize = std::thread::available_parallelism().map_or(1, |n| n.into());
    let lines_per_thread = pair_assignments.len() / num_threads;
    let (mut complete_overlap, mut partial_overlap) = (0, 0);

    thread::scope(|s| {
        let mut thread_handles: Vec<ScopedJoinHandle<(u32, u32)>> = Vec::with_capacity(num_threads);
        for line_chunk in pair_assignments.chunks(lines_per_thread) {
            thread_handles.push(s.spawn(|| contained_sections(line_chunk)));
        }
        for handle in thread_handles {
            let (part1, part2) = handle.join().unwrap();
            complete_overlap += part1;
            partial_overlap += part2;
        }
    });

    println!("{}", complete_overlap);
    println!("{}", partial_overlap);
    println!("ran for {} milliseconds on {} threads", now.elapsed().as_micros() as f64 / 1000.0, num_threads);
}
