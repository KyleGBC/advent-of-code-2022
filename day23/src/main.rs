use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap as HashMap, FxHashSet as HashSet};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction { North, East, South, West }
impl Direction {
    fn step(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (pos.0, pos.1 - 1),
            Direction::East => (pos.0 + 1, pos.1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::West => (pos.0 - 1, pos.1)
        }
    }
}

struct Grid {
    elves: HashSet<(i32, i32)>
}

impl Grid {
    fn apply_move(&mut self, old_pos: (i32, i32), new_pos: (i32, i32)) {
        if self.elves.remove(&old_pos) {
            self.elves.insert(new_pos);
        }
    }
    fn direction_contains_elf(&self, elf_pos: (i32, i32), direction: Direction) -> bool {
        let offsets: [(i32, i32); 3] = match direction {
            Direction::North => [(elf_pos.0, elf_pos.1 - 1), (elf_pos.0 - 1, elf_pos.1 - 1), (elf_pos.0 + 1, elf_pos.1 - 1)],
            Direction::East => [(elf_pos.0 + 1, elf_pos.1), (elf_pos.0 + 1, elf_pos.1 - 1), (elf_pos.0 + 1, elf_pos.1 + 1)],
            Direction::South => [(elf_pos.0, elf_pos.1 + 1), (elf_pos.0 - 1, elf_pos.1 + 1), (elf_pos.0 + 1, elf_pos.1 + 1)],
            Direction::West => [(elf_pos.0 - 1, elf_pos.1), (elf_pos.0 - 1, elf_pos.1 - 1), (elf_pos.0 - 1, elf_pos.1 + 1)]
        };

        for offset in offsets.iter() {
            if self.elves.contains(offset) {
                return true;
            }
        }
        false
    }
    
    fn bounding_box_score(&self) -> i32
    {
        let (min_x, max_x) = self.elves.iter().map(|(x, _)| x).minmax().into_option().unwrap();
        let (min_y, max_y) = self.elves.iter().map(|(_, y)| y).minmax().into_option().unwrap();

        (max_x - min_x + 1) * (max_y - min_y + 1) - self.elves.iter().count() as i32
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input: &str = include_str!("../input.txt");

    let mut grid = Grid{ elves: HashSet::with_capacity_and_hasher(1000, FxBuildHasher::default()) };
    let mut check_order = vec![Direction::North, Direction::South, Direction::West, Direction::East].into_iter().cycle();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.elves.insert((x as i32, y as i32));
            }
        }
    }

    let mut part1 = 0;
    let mut part2 = 10;
    for round in 1..
    {
        //let _ = std::io::stdin().read_line(&mut String::new());
        // Format of proposed_moves: (new_pos, old_pos or removed), for lookup reasons
        let mut proposed_moves: HashMap::<(i32, i32), Option<(i32, i32)>> = HashMap::with_capacity_and_hasher(grid.elves.len(), FxBuildHasher::default());
        for elf in grid.elves.iter().map(|e| *e)
        {
            let mut empty_directions = check_order.clone().take(4).filter(|d| !grid.direction_contains_elf(elf, *d));
            let num_options = empty_directions.clone().count();
            if num_options == 0 || num_options == 4 { continue; }
            
            let proposed_move = empty_directions.next().unwrap().step(elf);

            if proposed_moves.contains_key(&proposed_move)
            {
                *proposed_moves.get_mut(&proposed_move).unwrap() = None;
            }
            else { proposed_moves.insert(proposed_move, Some(elf)); }
        }
        check_order.next();

        let proposed_moves: Vec<_> = proposed_moves.drain().filter(|(_, v)| v.is_some()).map(|(k, v)| (k, v.unwrap())).collect();
        if proposed_moves.is_empty() { 
            part2 = round; break;
        }

        for (dest, src) in proposed_moves 
        {
            grid.apply_move(src, dest);
        }

        if round == 10 { part1 = grid.bounding_box_score(); }

    }

    println!("Part 1: {part1}, Part 2: {part2}, {:?}", now.elapsed());
}
