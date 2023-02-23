#[derive(Debug, Clone, Copy)]
enum Facing { Up, Down, Left, Right }
impl Facing {
    fn turn_left(self) -> Facing { match self { Self::Up => Self::Left, Self::Down => Self::Right, Self::Left => Self::Down, Self::Right => Self::Up } }
    fn turn_right(self) -> Facing { match self { Self::Up => Self::Right, Self::Down => Self::Left, Self::Left => Self::Up, Self::Right => Self::Down} } 
    fn value(self) -> usize { match self { Self::Up => 3, Self::Down => 1, Self::Left => 2, Self::Right => 0 } } 
}
#[derive(Debug, Clone, Copy)]
enum Tile { Open, Wall, Empty }
impl Tile { fn from_char(c: char) -> Self { match c { '.' => Self::Open, '#' => Self::Wall, ' ' => Self::Empty, _ => panic!("\"{}\" is not a valid board character", c) } } }

fn next_in_dir(board_size: (usize, usize), current_position: (usize, usize), dir: Facing) -> (usize, usize, Facing) {
    let next: (i32, i32) = match dir { Facing::Up => (current_position.0 as i32, current_position.1 as i32 - 1), Facing::Down => (current_position.0 as i32, current_position.1 as i32 + 1), Facing::Left => (current_position.0 as i32 - 1, current_position.1 as i32), Facing::Right => (current_position.0 as i32 + 1, current_position.1 as i32) };
    (next.0.rem_euclid(board_size.0 as i32) as usize, next.1.rem_euclid(board_size.1 as i32) as usize, dir)
}
fn next_on_cube(board: &Vec<Tile>, board_size: (usize, usize), mut current_position: (usize, usize), dir: Facing) -> (usize, usize, Facing) {
    unimplemented!();
}
fn step(board: &Vec<Tile>, board_size: (usize, usize), mut current_position: (usize, usize), mut dir: Facing, count: usize, cube: bool) -> (usize, usize, Facing) {
    let mut i = 0;
    let mut probe = current_position;
    while i != count {
        (probe.0, probe.1, dir) = if cube { next_on_cube(board, board_size, probe, dir) } else { next_in_dir(board_size, probe, dir) };
        let tile = board[probe.0 + probe.1*board_size.0];
        match tile {
            Tile::Open => { current_position = probe; i += 1 },
            Tile::Wall => { break }
            _ => {},
        }
    }
    (current_position.0, current_position.1, dir)
}
fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let (board_str, instructions) = input.split_once("\r\n\r\n").unwrap();
    let instructions = instructions.split_inclusive(|c: char| c.is_alphabetic()).collect::<Vec<&str>>();

    let (height, width) = (board_str.lines().count(), board_str.lines().map(|l| l.chars().count()).max().unwrap());
    let mut board = Vec::<Tile>::with_capacity(width * height);

    for line in board_str.lines() {
        let line_len = line.chars().count();
        for c in line.chars() {
            board.push(Tile::from_char(c));
        }
        (line_len..width).for_each(|_| board.push(Tile::Empty));
    }
    
    let i = board.iter().position(|t| matches!(t, Tile::Open)).unwrap();
    let mut p1_pos = (i % width, i / width);
    let mut p2_pos = p1_pos;
    let (mut p1_facing, mut p2_facing) = (Facing::Right, Facing::Right);

    for ins in instructions {
        let split_point = ins.find(|c: char| c.is_alphabetic()).unwrap_or(ins.len());
        let (count, turn): (&str, &str) = ins.split_at(split_point);
        let count = count.parse::<usize>().unwrap();
        (p1_pos.0, p1_pos.1, p1_facing) = step(&board, (width, height), p1_pos, p1_facing, count, false);
        //(p2_pos.0, p2_pos.1, p2_facing) = step(&board, (width, height), p2_pos, p2_facing, count, true);

        p1_facing = if turn == "L" { p1_facing.turn_left() } else if turn == "R" { p1_facing.turn_right() } else { p1_facing };
        p2_facing = if turn == "L" { p2_facing.turn_left() } else if turn == "R" { p2_facing.turn_right() } else { p2_facing };
    }
    let part1 = 1000*(p1_pos.1 + 1) + 4*(p1_pos.0 + 1) + p1_facing.value();
    let part2 = 1000*(p2_pos.1 + 1) + 4*(p2_pos.0 + 1) + p2_facing.value();

    println!("Part 1: {part1}, Part 2: {part2} in {:#?}", now.elapsed());
}