fn visible_trees<'a>(mut it: impl Iterator<Item = &'a u8>, tree_height: u8) -> (usize, bool) {
    let mut total = 0;
    let mut seen_from_edge = true;
    while let Some(tree) = it.next() {
        total += 1;
        if *tree >= tree_height {
            seen_from_edge = false;
            break;
        }
    } 
    (total, seen_from_edge)
} 
fn main() {
    let now = std::time::Instant::now();
    const GRID_SIZE: usize = 99;
    let mut grid = [[0_u8; GRID_SIZE]; GRID_SIZE];
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read in file");
    let (mut part1, mut part2) = (0_usize, 0_usize);

    

    for (y, line) in input.lines().enumerate() {
        for(x, char) in line.chars().enumerate() {
            grid[x][y] = char as u8 - 48;
        }
    }
    
    
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let tree_height = grid[x][y];
    
            let (above_trees, above_edge) = visible_trees(grid[x][..y].iter().rev(), tree_height);
            let (below_trees, below_edge) = visible_trees(grid[x][y+1..].iter(), tree_height);
            let (left_trees, left_edge) = visible_trees(grid[..x].iter().rev().map(|c| &c[y]), tree_height);
            let (right_trees, right_edge) = visible_trees(grid[x+1..].iter().map(|c| &c[y]), tree_height);

            let scenic = above_trees * below_trees * left_trees * right_trees;
            if scenic > part2 {
                part2 = scenic;
            }
            if above_edge || below_edge || left_edge || right_edge {
                part1 += 1;
            }
        }
    }
    println!("{} and {}, in {:#?}", part1, part2, now.elapsed());
}
