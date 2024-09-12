use std::{cell::RefCell, rc::Rc};
use rustc_hash::FxHashMap as HashMap;

fn wrap_between(value: i32, min: i32, max: i32) -> i32
{
    if value > max
    {
        let mut diff = value - max;
        diff = (diff - 1) % (max - min + 1);
        min + diff
    }
    else if value < min
    {
        let mut diff = min - value;
        diff = (diff - 1) % (max - min + 1);
        max - diff
    }
    else 
    {
        value
    }
}

fn taxicab_dist(a: (i32, i32), b: (i32, i32)) -> i32
{
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction { Up, Down, Left, Right }
impl Direction
{
    fn all() -> Vec<Direction> { vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] }
    fn step(&self, position: (i32, i32), count: i32) -> (i32, i32)
    {
        match self
        {
            Direction::Up => (position.0, position.1 - count),
            Direction::Down => (position.0, position.1 + count),
            Direction::Left => (position.0 - count, position.1),
            Direction::Right => (position.0 + count, position.1),
        }
    }
    fn from_char(c: char) -> Option<Direction>
    {
        match c
        {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Blizzard { initial_position: (i32, i32), direction: Direction }
#[derive(Debug, Clone)]
struct State<'a> 
{
    blizzards: &'a Vec<Blizzard>,
    valley_size: (i32, i32),
    expedition_position: (i32, i32),
    start_position: (i32, i32),
    goal_position: (i32, i32),
    time: i32,
    best_time_so_far: Rc<RefCell<i32>>,
    child_state_lut: Rc<RefCell<HashMap<((i32, i32), i32), i32>>>,
}

impl<'a> State<'a>
{
    fn new(blizzards: &Vec<Blizzard>, valley_size: (i32, i32)) -> State 
    {
        let child_state_lut = Rc::new(RefCell::new(HashMap::with_capacity_and_hasher((valley_size.0 * valley_size.1 * 500) as usize, Default::default())));
        let best_time_so_far = Rc::new(RefCell::new(i32::MAX));
        State { blizzards, valley_size, expedition_position: (1, 0), start_position: (1, 0), goal_position: (valley_size.0, valley_size.1 + 1), time: 0, child_state_lut, best_time_so_far }
    }

    fn child_states(&self) -> Vec<State> 
    {    
        let mut possible_moves = Direction::all().iter().map(|d| d.step(self.expedition_position, 1)).chain([self.expedition_position]).filter(|p| {
                    p.0 > 0 && p.0 <= self.valley_size.0 && p.1 > 0 && p.1 <= self.valley_size.1
                || *p == self.goal_position
                || *p == self.start_position
            }).collect::<Vec<_>>();

        for b in self.blizzards
        {
            let b_pos = b.direction.step(b.initial_position, self.time + 1); 
            let b_pos = (wrap_between(b_pos.0, 1, self.valley_size.0), wrap_between(b_pos.1, 1, self.valley_size.1));

            if let Some(idx) = possible_moves.iter().position(|p| *p == b_pos) 
            {
                possible_moves.swap_remove(idx);
            }
        }

        possible_moves.sort_unstable_by(|a, b| taxicab_dist(*a, self.goal_position).cmp(&taxicab_dist(*b, self.goal_position)));
        possible_moves.iter().map(|p| State {expedition_position: *p, time: self.time + 1, best_time_so_far: self.best_time_so_far.clone(), child_state_lut: self.child_state_lut.clone(), ..*self }).collect()
    }

    fn min_time_to_goal(&self) -> i32 
    {   
        if self.expedition_position == self.goal_position
        { 
            if self.time < *self.best_time_so_far.borrow() { 
                *self.best_time_so_far.borrow_mut() = self.time; 
            }
            self.time
        }
        else if self.child_state_lut.borrow().contains_key(&(self.expedition_position, self.time))
        {
            *self.child_state_lut.borrow().get(&(self.expedition_position, self.time)).unwrap()
        }
        else { 
            let mut children_min = i32::MAX;
            for s in self.child_states() 
            {
                let theoretical_best_possible_time = s.time + taxicab_dist(s.expedition_position, self.goal_position);
                if theoretical_best_possible_time > *self.best_time_so_far.borrow() { continue; }
                
                let actual_time = s.min_time_to_goal(); 
                
                self.child_state_lut.borrow_mut().insert((s.expedition_position, s.time), actual_time);
                children_min = children_min.min(actual_time);
            }
            children_min
        }
    }
}

fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../input.txt");
    
    let mut blizzards = Vec::new();

    for (y, line) in input.lines().enumerate()
    {
        for (x, c) in line.chars().enumerate()
        {
            if let Some(direction) = Direction::from_char(c)
            {
                blizzards.push(Blizzard { initial_position: (x as i32, y as i32), direction });
            }
        }
    }
    
    let valley_width = input.lines().next().unwrap().len() - 2;
    let valley_height = input.lines().count() - 2;

    let search_state = State::new(&blizzards, (valley_width as i32, valley_height as i32));
    
    let part1 = search_state.min_time_to_goal();

    println!("Part 1: {part1}, in {:?}", start.elapsed());

}
