#[derive(Debug, Clone, Copy, derive_more::derive::Display)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}
impl Facing {
    fn turn_left(self) -> Facing {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
    fn turn_right(self) -> Facing {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    fn value(self) -> usize {
        match self {
            Self::Up => 3,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pose {
    x: usize,
    y: usize,
    f: Facing,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Open,
    Wall,
    Empty,
}
impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '#' => Self::Wall,
            ' ' => Self::Empty,
            _ => panic!("\"{}\" is not a valid board character", c),
        }
    }
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    num_faces: (usize, usize),
    face_sizes: usize,
}
impl Board {
    fn step(&mut self, mut pose: Pose, count: usize, cube: bool) -> Pose {
        for _ in 0..count {
            let next = if cube {
                self.cube_next(pose)
            } else {
                self.flat_next(pose)
            };

            if matches!(self.tile_at(&next), Tile::Wall) {
                break;
            }
            pose = next;
        }
        pose
    }

    fn cube_next(&self, p: Pose) -> Pose {
        // If the next spot isn't an edge, the naive 2D next is correct
        let maybe_next = self.naive_next(p, false);

        if let Some(next) = maybe_next { 
            if !matches!(self.tile_at(&next), Tile::Empty) {
                return next;
            }
        }
    
        // Otherwise, use the hardcoded edge adjacencies for my specific cube
        // We could hope to build this from the 2D representation in the future as a hashmap
        use Facing as F;
        let (new_face, new_facing) = match (self.get_face_index(&p), p.f) {
            (1, F::Up) => (9, F::Right),
            (1, F::Left) => (6, F::Right),
            (2, F::Up) => (9, F::Up),
            (2, F::Right) => (7, F::Left),
            (2, F::Down) => (4, F::Left),
            (4, F::Right) => (2, F::Up),
            (4, F::Left) => (6, F::Down),
            (6, F::Up) => (4, F::Right),
            (6, F::Left) => (1, F::Right),
            (7, F::Right) => (2, F::Left),
            (7, F::Down) => (9, F::Left),
            (9, F::Right) => (7, F::Up),
            (9, F::Down) => (2, F::Down),
            (9, F::Left) => (1, F::Down),
            (i, f) => unimplemented!("{i}, {f} not found. This only holds for my specific cube layout"),
        };

        // This is the x-ness and y-ness in the specific edge of the current pose
        let (x_offset, y_offset) = (p.x % self.face_sizes, p.y % self.face_sizes);
        let (x_inset, y_inset) = ((self.face_sizes - 1) - x_offset, (self.face_sizes - 1) - y_offset);

        // From there we can figure out offset we need from the new face's corner to end up in the right spot
        let (x_offset, y_offset) = match (p.f, new_facing)
        {
            (F::Up, F::Up) | (F::Down, F::Down) | (F::Right, F::Left) | (F::Left, F::Right) => (x_offset, y_inset),
            (F::Up, F::Down) | (F::Down, F::Up) | (F::Left, F::Left) | (F::Right, F::Right)=> (x_inset, y_offset),
            (F::Up, F::Right) | (F::Down, F::Left) | (F::Right, F::Up) | (F::Left, F::Down) => (y_offset, x_offset),
            (F::Up, F::Left) | (F::Down, F::Right) | (F::Left, F::Up) | (F::Right, F::Down) => (y_inset, x_inset),        
        };

        let (x, y) = self.get_face_corner(new_face);
        Pose {
            x: x + x_offset,
            y: y + y_offset,
            f: new_facing,
        }
    }

    fn get_face_index(&self, p: &Pose) -> usize {
        p.x / self.face_sizes + p.y / self.face_sizes * self.num_faces.0
    }

    fn get_face_corner(&self, face_index: usize) -> (usize, usize) {
        let x = (face_index % self.num_faces.0) * self.face_sizes;
        let y = face_index / self.num_faces.0 * self.face_sizes;
        (x, y)
    }

    fn flat_next(&self, mut p: Pose) -> Pose
    {
        loop 
        {
            let next = self.naive_next(p, true).unwrap();
            if !matches!(self.tile_at(&next), Tile::Empty) {
                return next;
            }
            p = next;
        }
    }

    fn naive_next(&self, p: Pose, wrapping: bool) -> Option<Pose>
    {
        let (mut x, mut y): (i32, i32) = (p.x.try_into().unwrap(), p.y.try_into().unwrap());
        (x, y) = match p.f {
            Facing::Up => (x, y - 1),
            Facing::Down => (x, y + 1),
            Facing::Left => (x - 1, y),
            Facing::Right => (x + 1, y),
        };

        if wrapping
        {
            (x, y) = (x.rem_euclid(self.tiles[0].len() as i32), y.rem_euclid(self.tiles.len() as i32));
        }
        
        let (x, y) = (x.try_into().ok()?, y.try_into().ok()?);

        if x >= self.tiles[0].len() || y >= self.tiles.len()
        {
            return None;
        }
        Some(Pose { x, y, f: p.f })
    }

    fn tile_at(&self, p: &Pose) -> Tile {
        self.tiles[p.y][p.x]
    }
}

fn main() {
    let now = std::time::Instant::now();

    let input = include_str!("../input.txt");
    const FACE_SIZES: usize = 50;

    let (board_str, instructions) = input.split_once("\r\n\r\n").unwrap();
    let instructions = instructions
        .split_inclusive(|c: char| c.is_alphabetic())
        .collect::<Vec<&str>>();
    let (height, width) = (
        board_str.lines().count(),
        board_str.lines().map(|l| l.chars().count()).max().unwrap(),
    );

    let mut tiles: Vec<Vec<Tile>> = Vec::with_capacity(height);
    for line in board_str.lines() {
        let line_len = line.chars().count();
        let mut tile_line = Vec::with_capacity(width);

        line.chars().for_each(|c| {
            tile_line.push(Tile::from_char(c));
        });
        (line_len..width).for_each(|_| tile_line.push(Tile::Empty));

        tiles.push(tile_line);
    }

    let num_faces = (width / FACE_SIZES, height / FACE_SIZES);
    let mut board = Board {
        tiles,
        num_faces,
        face_sizes: FACE_SIZES,
    };

    let first_open = board
        .tiles
        .first()
        .unwrap()
        .iter()
        .position(|t| matches!(t, Tile::Open))
        .unwrap();
    let mut part1 = Pose {
        x: first_open,
        y: 0,
        f: Facing::Right,
    };
    let mut part2 = part1.clone();

    for ins in instructions {
        let (count, turn): (&str, &str) =
            ins.split_at(ins.find(|c: char| c.is_alphabetic()).unwrap_or(ins.len()));
        let count = count.parse::<usize>().unwrap();

        part1 = board.step(part1, count, false);
        part2 = board.step(part2, count, true);

        if turn == "L" {
            part1.f = part1.f.turn_left();
            part2.f = part2.f.turn_left();
        } else if turn == "R" {
            part1.f = part1.f.turn_right();
            part2.f = part2.f.turn_right();
        };
    }

    let part1 = (part1.y + 1) * 1000 + (part1.x + 1) * 4 + part1.f.value();
    let part2 = (part2.y + 1) * 1000 + (part2.x + 1) * 4 + part2.f.value();

    println!("Part 1: {part1}, Part 2: {part2} in {:#?}", now.elapsed());
}
