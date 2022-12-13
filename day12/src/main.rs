use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Node { pub distance_from_root: u32, pub elevation: u32, pub in_q: bool, pub prev: Option<(usize, usize)> }
impl Default for Node {
    fn default() -> Self {
        Self { distance_from_root: u32::MAX, elevation: Default::default(), in_q: true, prev: None }
    }
}

fn dijkstra(grid: &mut HashMap<(usize, usize), Node>, highest_point: (usize, usize)) {
    grid.get_mut(&highest_point).unwrap().distance_from_root = 0;
    while grid.values().any(|n| n.in_q) {
        let (cur_pos, u) = grid.iter_mut().filter(|a| a.1.in_q).min_by(|a, b| a.1.distance_from_root.cmp(&b.1.distance_from_root)).unwrap();
        let alt = u.distance_from_root.saturating_add(1);
        let cur_pos = cur_pos.clone();
        let cur_elevation = u.elevation.clone();
        u.in_q = false;

        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let child_pos = ((cur_pos.0 as i32 + dir.0) as usize, (cur_pos.1 as i32 + dir.1) as usize);
            if let Some(child) = grid.get_mut(&child_pos) {
                if child.in_q && alt < child.distance_from_root && cur_elevation as i32 - child.elevation as i32 <= 1 {
                        child.distance_from_root = alt;
                        child.prev = Some(cur_pos);
                }
            }
        }
    }
}
fn distance(grid: &HashMap<(usize, usize), Node>, mut current_point: (usize, usize), highest_point: (usize, usize)) -> usize {
    let mut dis = 0;
    if let Some(_) = grid.get(&current_point).unwrap().prev {
        while current_point != highest_point {
            current_point = grid.get(&current_point).unwrap().prev.unwrap();
            dis += 1;
        }
    }
    dis
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let mut grid: HashMap<(usize, usize), Node> = HashMap::with_capacity(114 * 114);
    let mut root = (0, 0);
    let mut trailheads: Vec<(usize, usize)> = Vec::with_capacity(100);
    let mut target = (0, 0);
    
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut new_node = Node::default();
            match c {
                'S' => { root = (x, y); new_node.elevation = 1 },
                'E' => { target = (x, y); new_node.elevation = 26},
                'a' => { trailheads.push((x, y)); new_node.elevation = 1},
                _ => new_node.elevation = c as u32 - 96
            }
            grid.insert((x, y), new_node);
        }
    }
    
    dijkstra(&mut grid, target);
    let part1 = distance(&grid, root, target);
    let part2 = trailheads.iter().map(|t| distance(&grid, *t, target)).filter(|n| *n!=0).min().unwrap();
    println!("Part 1: {part1}, Part 2: {:#?}, in {:#?}", part2, now.elapsed());
    
}