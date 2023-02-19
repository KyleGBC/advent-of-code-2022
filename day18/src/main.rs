use std::{collections::{HashMap, HashSet, VecDeque}, time::Instant, hash::BuildHasherDefault };
use hashers::fx_hash::FxHasher;

#[derive(PartialEq, Eq, Debug)]
enum Mat { Lava, Air }
type Coord = (i32, i32, i32);
type FxHash = BuildHasherDefault<FxHasher>;

fn get_adjacent(data: &HashMap<Coord, Mat, FxHash>, root: Coord) -> Vec<Coord> {
    let mut ret = Vec::with_capacity(6);
    let (x, y, z) = root;

    if data.get(&(x + 1, y, z)) == Some(&Mat::Air) { ret.push((x + 1, y, z)) }
    if data.get(&(x - 1, y, z)) == Some(&Mat::Air) { ret.push((x - 1, y, z)) }
    if data.get(&(x, y + 1, z)) == Some(&Mat::Air) { ret.push((x, y + 1, z)) }
    if data.get(&(x, y - 1, z)) == Some(&Mat::Air) { ret.push((x, y - 1, z)) }
    if data.get(&(x, y, z + 1)) == Some(&Mat::Air) { ret.push((x, y, z + 1)) }
    if data.get(&(x, y, z - 1)) == Some(&Mat::Air) { ret.push((x, y, z - 1)) }

    ret
}

fn is_exterior_air(data: &HashMap<Coord, Mat, FxHash>, root: Coord) -> bool {
    if data.get(&root) == Some(&Mat::Lava) { return false }

    let mut exploration: HashSet<Coord, FxHash> = HashSet::with_capacity_and_hasher(20_000, FxHash::default());
    let mut queue: VecDeque<Coord> = VecDeque::with_capacity(20_000);

    exploration.insert(root);
    queue.push_back(root);
    while !queue.is_empty() {   
        let v = queue.pop_front().unwrap();
        if v == (0, 0, 0) {
            return true;
        }
        for adj in get_adjacent(data, v) {
            let adj = adj;
            if !exploration.contains(&adj) {
                exploration.insert(adj);
                queue.push_back(adj)
            }
        }
    }
    false
}

fn part1(data: &HashMap<Coord, Mat, FxHash>) -> i32 {
    let mut total: i32 = 0;
    for (coord, mat) in data {
        if mat == &Mat::Air { continue }
        total += get_adjacent(data, *coord).len() as i32
    }
    total
}

fn part2(data: &HashMap<Coord, Mat, FxHash>) -> i32 {
    let mut total = 0;
    for (coord, mat) in data {
        if mat == &Mat::Air { continue }
    
        for adj in get_adjacent(data, *coord) {
            if is_exterior_air(data, adj) { total += 1 }
        }
    }
    total
}

fn main() {
    let time = Instant::now();
    let input = include_str!("../input.txt");
    let mut data: HashMap<Coord, Mat, FxHash> = HashMap::with_capacity_and_hasher(100, FxHash::default());

    // Get all the lava points
    for line in input.lines() {
        let v: Vec<_> = line.split(',').collect();
        data.insert((v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap(), v[2].parse::<i32>().unwrap()), Mat::Lava);
    }
    
    // Find the bounds of the space
    let max_x = data.iter().map(|((x, _, _), _)| *x).max().unwrap();
    let max_y = data.iter().map(|((_, y, _), _)| *y).max().unwrap();
    let max_z = data.iter().map(|((_, _, z), _)| *z).max().unwrap();

    // Tag the remaining parts as Air
    for x in -1..=(max_x+1) {
        for y in -1..=(max_y+1) {
            for z in -1..=(max_z+1) {
                if !data.contains_key(&(x, y, z)) { data.insert((x, y, z), Mat::Air); }
            }
        }
    }

    let sa = part1(&data);
    let true_sa = part2(&data);
    println!("Part 1 is {}, Part 2 is {}, in {:#?}", sa, true_sa, time.elapsed());
}