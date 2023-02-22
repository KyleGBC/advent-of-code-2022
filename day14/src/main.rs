fn fill_between(first: &str, second: &str, set: &mut [[bool; 200]; 1000]) {
    let first = first.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let second = second.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (lower_x, higher_x) = if first[0] < second[0] { (first[0], second[0]) } else { (second[0], first[0]) };
    let (lower_y, higher_y) = if first[1] < second[1] { (first[1], second[1]) } else { (second[1], first[1]) };
    for x in lower_x..=higher_x {
        for y in lower_y..=higher_y {
            set[x][y] = true
        }
    }
}

fn simulate_sand(mut rocks: [[bool; 200]; 1000], lowest: usize, floor: bool) -> usize {
    if floor { rocks.iter_mut().for_each(|c| c[lowest + 2] = true) }
    let mut sand_count = 0;
    'sand: loop {
        let mut sand: (usize, usize) = (500, 0);
        'fall: loop {
            if !rocks[sand.0][sand.1 + 1] { sand.1 += 1 }
            else if !rocks[sand.0 - 1][sand.1 + 1] { sand.0 -= 1; sand.1 += 1 }
            else if !rocks[sand.0 + 1][sand.1 + 1] { sand.0 += 1; sand.1 += 1 }
            else if floor && sand == (500, 0) { sand_count += 1; break 'sand }
            else { rocks[sand.0][sand.1] = true; break 'fall }

            if sand.1 > lowest && !floor { break 'sand } 
        }
        sand_count += 1;
    }
    sand_count
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let mut rocks = [[false; 200]; 1000];

    for line in input.lines() {
        for endpoints in line.split(" -> ").collect::<Vec<&str>>().windows(2) {
            if let [first, second] = endpoints {
                fill_between(first, second, &mut rocks);
            }
        }
    }

    let lowest = rocks.iter().map(|c| c.iter().enumerate().filter(|(_, f)| **f).map(|(i, _)| i).max().unwrap_or(0)).max().unwrap();
    let part1 = simulate_sand(rocks, lowest, false);
    let part2 =  simulate_sand(rocks, lowest, true);

    println!("Part 1: {part1}, Part 2: {part2} in {:#?}", now.elapsed());
}
