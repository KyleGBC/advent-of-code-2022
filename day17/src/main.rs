use std::collections::{HashMap, VecDeque};
type State = (VecDeque<[bool; 7]>, usize, Shape);

const GRID_HEIGHT: usize = 10_000;
#[derive(Debug, PartialEq, Eq, Hash)]
enum Shape{ Plus, L, Horizontal, Square, Vertical }
impl Shape { 
    fn from_num(n: usize) -> Shape { match n { 1 => Shape::Horizontal, 2 => Shape::Plus, 3 => Shape::L, 4 => Shape::Vertical, 5 => Shape::Square, _ => panic!() } } 
    fn width(&self) -> usize { match self { Shape::Horizontal => 4, Shape::Plus | Shape::L => 3, Shape::Square => 2, Shape::Vertical => 1} }
    fn height(&self) -> usize { match self { Shape::Horizontal => 1, Shape::Plus | Shape::L => 3, Shape::Square => 2, Shape::Vertical => 4 } }
}
#[derive(Debug)]
struct Rock { shape: Shape, x: usize, y: usize}
impl Rock {
    fn new(shape: Shape, y: usize) -> Rock {
        Rock {shape, x: 2, y}
    }
    fn push_left(&mut self, grid: &[[bool; GRID_HEIGHT]; 7]) {
        if self.x > 0 {
            match self.shape {
                Shape::Vertical => if !(0..4).any(|n| grid[self.x - 1][self.y + n]) {self.x -= 1}
                Shape::Horizontal => if !grid[self.x - 1][self.y] { self.x -= 1 }
                Shape::Square => if !(0..2).any(|n| grid[self.x - 1][self.y + n]) { self.x -= 1 }
                Shape::L => if !grid[self.x - 1][self.y] && !(1..3).any(|n| grid[self.x + 1][self.y + n]) { self.x -= 1 } 
                Shape::Plus => if !grid[self.x][self.y] && !grid[self.x - 1][self.y + 1] && !grid[self.x][self.y + 2] { self.x -= 1}
            }
        }
    }
    fn push_right(&mut self, grid: &[[bool; GRID_HEIGHT]; 7]) {
        let w = self.shape.width();
        if self.x + w < 7 {
            match self.shape {
                Shape::Vertical => if !(0..4).any(|n| grid[self.x + w][self.y + n]) { self.x += 1 }
                Shape::Horizontal => if !grid[self.x + w][self.y] { self.x += 1 }
                Shape::Square => if !(0..2).any(|n| grid[self.x + w][self.y + n]) { self.x += 1 }
                Shape::L => if !(0..3).any(|n| grid[self.x + w][self.y + n]) { self.x += 1 }
                Shape::Plus => if !grid[self.x + w - 1][self.y] && !grid[self.x + w][self.y + 1] && !grid[self.x + w - 1][self.y + 2] { self.x += 1 }
            }
        }
    }
    fn fall(&mut self, grid: &[[bool; GRID_HEIGHT]; 7]) -> bool {
        if self.y == 1 {
            false
        }
        else {
            match self.shape {
                Shape::Vertical => if !grid[self.x][self.y - 1] { self.y -= 1; true } else { false } 
                Shape::Horizontal => if !(0..4).any(|n| grid[self.x + n][self.y - 1]) { self.y -= 1; true } else { false }
                Shape::Square => if !(0..2).any(|n| grid[self.x + n][self.y - 1]) { self.y -= 1; true } else { false }
                Shape::L => if !(0..3).any(|n| grid[self.x + n][self.y - 1]) { self.y -= 1; true } else { false }
                Shape::Plus => if !grid[self.x][self.y] && !grid[self.x + 1][self.y - 1] && !grid[self.x + 2][self.y] { self.y -= 1; true} else { false }
            }
        }
    }
    fn add_to_grid(&self, grid: &mut[[bool; GRID_HEIGHT]; 7]) {
        match self.shape {
            Shape::Vertical => (0..4).for_each(|n| grid[self.x][self.y + n] = true),
            Shape::Horizontal => (0..4).for_each(|n| grid[self.x + n][self.y] = true),
            Shape::Square => (0..2).for_each(|n| (0..2).for_each(|m| grid[self.x + n][self.y + m] = true)),
            Shape::L => (0..3).for_each(|n| { grid[self.x + n][self.y] = true; grid[self.x + 2][self.y + n] = true } ),
            Shape::Plus => (0..3).for_each(|n| { grid[self.x + 1][self.y + n] = true; grid[self.x + n][self.y + 1] = true })
        } 
    }
}


fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let mut jets = input.chars().enumerate().cycle();
    let mut grid = [[false; GRID_HEIGHT]; 7];
    
    let mut seen_states: HashMap<State, (usize, usize)> = HashMap::with_capacity(10000);
    let mut previous_floors: VecDeque<[bool; 7]> = VecDeque::new();

    let mut highest_point = 0;
    let mut cycle_mod = 0;
    let mut rock_count = 0_usize;
    for shape in (1..=5).cycle().map(Shape::from_num) {
        let mut rock = Rock::new(shape, highest_point + 4);
        loop {
            let (jet_index, push_dir) = jets.next().unwrap();
            match push_dir {
                '<' => { rock.push_left(&grid) },
                '>' => { rock.push_right(&grid) },
                _ => unreachable!()
            }
            if !rock.fall(&grid) {
                rock.add_to_grid(&mut grid);
                rock_count += 1;
                highest_point = usize::max(highest_point, rock.y + rock.shape.height() - 1);

                if cycle_mod == 0 {
                    previous_floors.push_back([grid[0][highest_point], grid[1][highest_point], grid[2][highest_point], grid[3][highest_point], grid[4][highest_point], grid[5][highest_point], grid[6][highest_point]]);
                    if previous_floors.len() > 100 {
                        previous_floors.pop_front();
                    }
                    if let Some((prev_count, prev_highest)) = seen_states.insert((previous_floors.clone(), jet_index, rock.shape), (rock_count, highest_point)) {
                        let jumps = (1000000000000 - rock_count) / (rock_count - prev_count);
                        rock_count += jumps * (rock_count - prev_count);
                        cycle_mod = jumps * (highest_point - prev_highest);
                    }
                }
                break
            }
        }
        if rock_count == 2022 {
            println!("Part 1: {highest_point}, in {:#?}", now.elapsed());
        }
        if rock_count == 1000000000000 {
            println!("Part 2: {}, in {:#?} overall", highest_point + cycle_mod, now.elapsed());
            break;
        }
    }


}
