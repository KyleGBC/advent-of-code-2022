use std::{collections::HashMap, hash::BuildHasherDefault};
use hashers::fx_hash::FxHasher;
type FxHash = BuildHasherDefault<FxHasher>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op { Add, Sub, Mul, Div }
impl Op { fn perform_op(&self, a: i64, b: i64) -> i64 { match self { Op::Add => a + b, Op::Sub => a - b, Op::Mul => a * b, Op::Div => a / b } } }
#[derive(Debug, PartialEq)]
enum Yell { Imm(i64), Parent(Box<(Yell, Op, Yell)>), Str(&'static str, Op, &'static str) }
impl Yell {
    fn from_str(s: &'static str) -> Self {
        if s.chars().filter(|c| c.is_numeric()).count() != 0 {
            Yell::Imm(s.parse::<i64>().unwrap())
        }
        else {
            let args = s.split_whitespace().collect::<Vec<_>>();
            let o = match args[1] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div, 
                _ => unreachable!("\"{}\" is not a valid math operator", args[1])
            };
            Yell::Str(args[0], o, args[2])
        }
    }
    fn value(&self) -> i64 {
        match self {
            Yell::Str(..) => panic!("Can't take the value of unlinked Yell"),
            Yell::Imm(n) => *n, 
            Yell::Parent(b) => { b.1.perform_op(b.0.value(), b.2.value()) }
        }
    }
}

fn attach_branches(known_values: &mut HashMap<&str, Yell, FxHash>, id: &str) { 
    if let Some(Yell::Str(oper_1, op, oper_2)) = known_values.get(id) {
        let (oper_1, op, oper_2) = (*oper_1, *op, *oper_2);
        let (yell_1, yell_2) = (known_values.remove(oper_1).unwrap(), known_values.remove(oper_2).unwrap());
        if !matches!(yell_1, Yell::Str(..)) && !matches!(yell_2, Yell::Str(..)) {
            *known_values.get_mut(id).unwrap() = Yell::Parent(Box::new((yell_1, op, yell_2)));
        }
        else {
            known_values.insert(oper_1, yell_1);
            known_values.insert(oper_2, yell_2);
        }
    }   
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    
    let mut known_values: HashMap<&str, Yell, FxHash> = HashMap::with_capacity_and_hasher(input.lines().count(), FxHash::default());

    for line in input.lines() {
        let (id, yell_str) = line.split_once(": ").unwrap();
        known_values.insert(id, Yell::from_str(yell_str));
    }
    let monkey_ids = known_values.keys().cloned().collect::<Vec<_>>();

    loop {
        if matches!(known_values.get("root").unwrap(), Yell::Parent(..)) { break }
        for id in monkey_ids.iter() {
            attach_branches(&mut known_values, id);
        }
    }
    let root = known_values.remove("root").unwrap();

    let part1 = root.value();

    println!("Part 1: {part1}, in {:#?}", now.elapsed());
}