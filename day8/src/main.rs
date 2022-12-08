fn main() {
    const GRID_SIZE: usize = 99;
    let mut grid = [[0_u8; GRID_SIZE]; GRID_SIZE];
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read in file");
    let mut part1 = 0_usize;
    let mut part2 = 0_usize;

    for (y, line) in input.lines().enumerate() {
        for(x, char) in line.chars().enumerate() {
            grid[x][y] = char as u8 - 48;
        }
    }

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let tree_height = grid[x][y];

            let above = &grid[x][..y].iter().all(|h| h < &tree_height);
            let below = &grid[x][y+1..GRID_SIZE].iter().all(|h| h < &tree_height);
            let left = &grid[..x].iter().all(|c| c[y] < tree_height);
            let right = &grid[x+1..GRID_SIZE].iter().all(|c| c[y] < tree_height);
            part1 += (*above || *below || *left || *right) as usize;

            let above_distance = &grid[x][..y].iter().rev().position(|h| h < &tree_height).unwrap_or(y);
            let below_distance = &grid[x][y+1..GRID_SIZE].iter().position(|h| h < &tree_height).unwrap_or(GRID_SIZE - y);
            let left_distance = &grid[..x].iter().rev().position(|c| c[y] < tree_height).unwrap_or(x);
            let right_distance = &grid[x+1..GRID_SIZE].iter().position(|c| c[y] < tree_height).unwrap_or(GRID_SIZE - x);
            let scenic = *right_distance * *left_distance * *above_distance * *below_distance;
            if scenic > part2 {
                part2 = scenic;
            }

        }
    }
    println!("{part1}, {part2}");
}
