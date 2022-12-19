use std::collections::{HashSet, VecDeque};

fn adjacent_air(current: (usize, usize, usize), other_cubes: &HashSet<(usize, usize, usize)>) -> Vec<(usize, usize, usize)> {
    let mut adjacent = Vec::with_capacity(6);
    if !other_cubes.contains(&(current.0 + 1, current.1, current.2)) { adjacent.push((current.0 + 1, current.1, current.2)) }
    if !other_cubes.contains(&(current.0, current.1 + 1, current.2)) { adjacent.push((current.0, current.1 + 1, current.2)) }
    if !other_cubes.contains(&(current.0, current.1, current.2 + 1)) { adjacent.push((current.0, current.1, current.2 + 1)) }
    if !other_cubes.contains(&(current.0 - 1, current.1, current.2)) { adjacent.push((current.0 - 1, current.1, current.2)) }
    if !other_cubes.contains(&(current.0, current.1 - 1, current.2)) { adjacent.push((current.0, current.1 - 1, current.2)) }
    if !other_cubes.contains(&(current.0, current.1, current.2 - 1)) { adjacent.push((current.0, current.1, current.2 - 1)) }
    adjacent
}


fn bfs(start: (usize, usize, usize), goal: (usize, usize, usize), other_cubes: &HashSet<(usize, usize, usize)>) -> HashSet<(usize, usize, usize)> {
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut explored: HashSet<(usize, usize, usize)> = HashSet::new();
    explored.insert(start);

    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        for w in adjacent_air(v, other_cubes) {
            if !explored.contains(&w) {
                explored.insert(w);
                q.push_back(w)
            }
        }
    }
    return explored;
}

fn surface_area(cube: (usize, usize, usize), other_cubes: &HashSet<(usize, usize, usize)>) -> usize {
    let mut surface_area = 6;
    if other_cubes.contains(&(cube.0 + 1, cube.1, cube.2)) { surface_area -= 1 }
    if other_cubes.contains(&(cube.0, cube.1 + 1, cube.2)) { surface_area -= 1 }
    if other_cubes.contains(&(cube.0, cube.1, cube.2 + 1)) { surface_area -= 1 }
    
    if let Some(n) = cube.0.checked_sub(1) {
        if other_cubes.contains(&(n, cube.1, cube.2)) { surface_area -= 1 }
    }
    if let Some(n) = cube.1.checked_sub(1) {
        if other_cubes.contains(&(cube.0, n, cube.2)) { surface_area -= 1 }
    }
    if let Some(n) = cube.2.checked_sub(1) {
        if other_cubes.contains(&(cube.0, cube.1, n)) { surface_area -= 1 }
    }
    surface_area
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../sample.txt");
    let mut cubes: HashSet<(usize, usize, usize)> = HashSet::with_capacity(2025);
    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);
    let (mut min_x, mut min_y, mut min_z) = (usize::MAX, usize::MAX, usize::MAX);
    let mut max_y = 0;
    let mut max_z = 0;
    let mut min_x = usize::MAX;
    let mut min_z = usize::MAX;
    let mut min_y = usize::MAX;
    for line in input.lines() {
        let mut pos = line.split(',');
        let (x, y, z) = ( pos.next().unwrap().parse::<usize>().unwrap(),
                                           pos.next().unwrap().parse::<usize>().unwrap(),
                                           pos.next().unwrap().parse::<usize>().unwrap() );
        max_x = usize::max(max_x, x);
        max_y = usize::max(max_y, y);
        max_z = usize::max(max_z, z);
        min_x = usize::min(min_x, x);
        min_y = usize::min(min_y, y);
        min_z = usize::min(min_z, z);
        cubes.insert((x, y, z));
    }

    let mut filled = cubes.clone();
    let max_x = 
    for x in 1..20 {
        for y in 1..20 {
            for z in 1..20 {
                if !cubes.contains(&(x, y, z)) {
                    println!("Air pocket found at {},{},{}", x,y, z);
                    filled.insert((x, y, z));
                }
            }
        }
    }

    let part1 = cubes.iter().fold(0, |acc, c| acc + surface_area(*c, &cubes));
    let part2 = filled.iter().fold(0, |acc, c| acc + surface_area(*c, &filled));
    println!("Part 1: {part1}, Part 2: {part2}, in {:#?}", now.elapsed())
}
