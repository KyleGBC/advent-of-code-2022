use std::collections::HashSet;

fn visible_from_outside(cube: (usize, usize, usize), other_cubes: &HashSet<(usize, usize, usize)>) -> bool {
    let pos_x = (0..cube.0).all(|n| !other_cubes.contains(&(n, cube.1, cube.2)));
    let pos_y = (0..cube.1).all(|n| !other_cubes.contains(&(cube.0, n, cube.2)));
    let pos_z = (0..cube.2).all(|n| !other_cubes.contains(&(cube.0, cube.1, n)));
    let neg_x = (cube.0+1..20).all(|n| !other_cubes.contains(&(n, cube.1, cube.2)));
    let neg_y = (cube.1+1..20).all(|n| !other_cubes.contains(&(cube.0, n, cube.2)));
    let neg_z = (cube.2+1..20).all(|n| !other_cubes.contains(&(cube.0, cube.1, n)));
    pos_x || pos_y || pos_z || neg_x || neg_y || neg_z
}


fn exterior_surface_area(cube: (usize, usize, usize), other_cubes: &HashSet<(usize, usize, usize)>) -> usize {
    let mut surface_area = 0;
    if visible_from_outside((cube.0 + 1, cube.1, cube.2), &other_cubes) && !other_cubes.contains(&(cube.0 + 1, cube.1, cube.2)) { surface_area += 1 }
    if visible_from_outside((cube.0 - 1, cube.1, cube.2), &other_cubes) && !other_cubes.contains(&(cube.0 - 1, cube.1, cube.2)) { surface_area += 1 }
    if visible_from_outside((cube.0, cube.1 + 1, cube.2), &other_cubes) && !other_cubes.contains(&(cube.0, cube.1 + 1, cube.2)) { surface_area += 1 }
    if visible_from_outside((cube.0, cube.1 - 1, cube.2), &other_cubes) && !other_cubes.contains(&(cube.0, cube.1 - 1, cube.2)) { surface_area += 1 }
    if visible_from_outside((cube.0, cube.1, cube.2 + 1), &other_cubes) && !other_cubes.contains(&(cube.0, cube.1, cube.2 + 1)) { surface_area += 1 }
    if visible_from_outside((cube.0, cube.1, cube.2 - 1), &other_cubes) && !other_cubes.contains(&(cube.0, cube.1, cube.2 - 1)) { surface_area += 1 }
    surface_area    
}  


fn surface_area(cube: (usize, usize, usize), other_cubes: &HashSet<(usize, usize, usize)>) -> usize {
    let mut surface_area = 6;
    if other_cubes.contains(&(cube.0 + 1, cube.1, cube.2)) { surface_area -= 1 }
    if other_cubes.contains(&(cube.0 - 1, cube.1, cube.2)) { surface_area -= 1 }
    if other_cubes.contains(&(cube.0, cube.1 + 1, cube.2)) { surface_area -= 1 }
    if other_cubes.contains(&(cube.0, cube.1 - 1, cube.2)) { surface_area -= 1 }
    if other_cubes.contains(&(cube.0, cube.1, cube.2 + 1)) { surface_area -= 1 }
    if other_cubes.contains(&(cube.0, cube.1, cube.2 - 1)) { surface_area -= 1 } 
    surface_area
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let mut cubes: HashSet<(usize, usize, usize)> = HashSet::with_capacity(2025);
    for line in input.lines() {
        let mut pos = line.split(',');
        let pos = ( pos.next().unwrap().parse::<usize>().unwrap(),
                                           pos.next().unwrap().parse::<usize>().unwrap(),
                                           pos.next().unwrap().parse::<usize>().unwrap() );
        cubes.insert(pos);
    }

    let part1 = cubes.iter().fold(0, |acc, c| acc + surface_area(*c, &cubes));
    let part2 = cubes.iter().fold(0, |acc, c| acc + exterior_surface_area(*c, &cubes));
    println!("Part 1: {part1}, Part 2: {part2}, in {:#?}", now.elapsed())
}
