use std::{collections::HashMap, hash::BuildHasherDefault};
use hashers::fx_hash::FxHasher;
type FxHash = BuildHasherDefault<FxHasher>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op { Add, Sub, Mul, Div }
impl Op { fn perform_op(&self, a: i64, b: i64) -> i64 { match self { Op::Add => a + b, Op::Sub => a - b, Op::Mul => a * b, Op::Div => a / b } } }
#[derive(Debug, PartialEq)]
enum Yell { Imm(i64), Parent(Box<(Yell, Op, Yell)>), Human(i64) }
impl Yell {
    fn from_str(id: &'static str, known_values: &HashMap<&'static str, &'static str, FxHash>) -> Self {
        let s = known_values.get(id).unwrap();
        if s.chars().filter(|c| c.is_numeric()).count() > 0 {
            let n = s.parse::<i64>().unwrap();
            if id == "humn" { Yell::Human(n) } else { Yell::Imm(n) }
        }
        else {
            let args = s.splitn(3, ' ').collect::<Vec<_>>();
            let op = match args[1] {"+" => Op::Add, "-" => Op::Sub, "*" => Op::Mul, "/" => Op::Div, _ => panic!("\"{}\" is not a valid math operator", args[1]) };
            Yell::Parent(Box::new((Yell::from_str(args[0], known_values), op, Yell::from_str(args[2], known_values))))
        }
    }
    fn into_children(self) -> (Yell, Op, Yell) { if let Yell::Parent(b) = self { *b } else { panic!("attempted to get children of a non-parent") } }
    fn value(&self) -> i64 {
        match self {
            Yell::Imm(n) | Yell::Human(n) => *n, 
            Yell::Parent(b) => { b.1.perform_op(b.0.value(), b.2.value()) }
        }
    }
    fn make_immediates(&mut self) {
        if let Yell::Parent(b) = self  {
                let (lhs, op, rhs) = b.as_mut();
                lhs.make_immediates();
                rhs.make_immediates();
                if let (Yell::Imm(a), Yell::Imm(b)) = (lhs, rhs) {
                    *self = Yell::Imm(op.perform_op(*a, *b));
                }
            }
    }
    fn undo_op(self, h: &mut i64) -> Yell {
        let (lhs, op, rhs)  = self.into_children();
        match op {
            Op::Add =>{
                let (cons, var) = if matches!(lhs, Yell::Imm(..)) { (lhs, rhs) } else { (rhs, lhs) };
                *h -= cons.value();
                var
            },
            Op::Sub => {
                if let Yell::Imm(n) = lhs {
                    *h = n - *h;
                    rhs
                }
                else if let Yell::Imm(n) = rhs {
                    *h += n;
                    lhs
                }
                else { unreachable!() }
            },
            Op::Mul => {
                let (cons, var) = if matches!(lhs, Yell::Imm(..)) { (lhs, rhs) } else { (rhs, lhs) };
                *h /= cons.value();
                var
            },
            Op::Div => {
                if let Yell::Imm(n) = lhs {
                    *h = n / *h;
                    rhs
                }
                else if let Yell::Imm(n) = rhs {
                    *h *= n;
                    lhs
                }
                else { unreachable!() }
            },
        }
    }
    fn solve_for_humn(mut self) -> i64 {
        self.make_immediates();
        let (lhs, _, rhs) = self.into_children();
        let (constant_side, mut human_side) = if matches!(lhs, Yell::Imm(..)) { (lhs, rhs) } else { (rhs, lhs) };
        let mut h = constant_side.value();
        while !matches!(human_side, Yell::Human(..)) {
            human_side = human_side.undo_op(&mut h);
        }
        h 
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    
    let mut known_values: HashMap<&str, &str, FxHash> = HashMap::with_capacity_and_hasher(input.lines().count(), FxHash::default());

    for line in input.lines() {
        let (id, yell_str) = line.split_once(": ").unwrap();
        known_values.insert(id, yell_str);
    }

    let root = Yell::from_str("root", &known_values);
    let part1 = root.value();
    let part2 = root.solve_for_humn();

    println!("Part 1: {part1}, Part 2: {part2}, in {:#?}", now.elapsed());
}