use std::hash::BuildHasherDefault;
use std::collections::HashSet;
use hashers::fx_hash::FxHasher;

fn update_knot(leader: (i32, i32), follower: &mut (i32, i32)) {
    let (x_dist, y_dist) = (leader.0 - follower.0, leader.1 - follower.1);
    if x_dist.abs() > 1 || y_dist.abs() > 1 {
        follower.0 += x_dist.signum();
        follower.1 += y_dist.signum();
    }
}

fn run_with_n_knots<const N: usize>(lines: std::str::Lines) -> (usize, usize) {
    let mut knot_positions = [(0_i32, 0_i32); N];
    let mut tail_positions: HashSet<(i32, i32), BuildHasherDefault<FxHasher>> = HashSet::with_capacity_and_hasher(10000, BuildHasherDefault::<FxHasher>::default());
    let mut behind_head_positions: HashSet<(i32, i32), BuildHasherDefault<FxHasher>> = HashSet::with_capacity_and_hasher(10000, BuildHasherDefault::<FxHasher>::default());

    for line in lines {
        let (dir, step) = line.split_once(' ').unwrap();
        let step = step.parse::<u32>().unwrap();
        for _ in 0..step {
            match dir {
                "U" => knot_positions[0].1 += 1,
                "D" => knot_positions[0].1 -= 1,
                "L" => knot_positions[0].0 += 1,
                "R" => knot_positions[0].0 -= 1,
                _ => unreachable!(),
            }
            for l in 0..N - 1 {
                update_knot(knot_positions[l], &mut knot_positions[l + 1_usize]);
            }
            tail_positions.insert(knot_positions[N - 1]);
            behind_head_positions.insert(knot_positions[1]);
        }           
    }
    (behind_head_positions.len(), tail_positions.len())
}

fn main() {
    let now = std::time::Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{:?} in {:#?}", run_with_n_knots::<10>(input.lines()), now.elapsed());
}
