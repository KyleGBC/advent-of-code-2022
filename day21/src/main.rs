use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op { Add, Sub, Mul, Div }
impl Op { fn perform_op(&self, a: i64, b: i64) -> i64 { match self { Op::Add => a + b, Op::Sub => a - b, Op::Mul => a * b, Op::Div => a / b } } }
#[derive(Debug, PartialEq)]
enum Yell { Imm(i64), Parent(Box<(Yell, Op, Yell)>), Str(String, Op, String) }
impl Yell {
    fn from_str(s: &str) -> Self {
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
            Yell::Str(args[0].to_string(), o, args[2].to_string())
        }
    }
    fn is_str(&self) -> bool { match self { &Yell::Str(..) => true, _ => false }}
    fn is_parent(&self) -> bool { match self {&Yell::Parent(..) => true, _ => false }}
    fn value(&self) -> i64 {
        match self {
            Yell::Str(..) => panic!("Can't take the value of unlinked Yell"),
            Yell::Imm(n) => *n, 
            Yell::Parent(b) => { b.1.perform_op(b.0.value(), b.2.value()) }
        }
    }
}

fn attach_branches(known_values: &mut HashMap<String, Yell>, id: &String) {
    if let Some(mut old_yell) = known_values.remove(id) {
        if let Yell::Str(oper_1, op, oper_2) = &old_yell {
            if let (Some(yell_1), Some(yell_2)) = (known_values.remove(oper_1), known_values.remove(oper_2)) {
                if !yell_1.is_str() && !yell_2.is_str() {
                    old_yell = Yell::Parent(Box::new((yell_1, *op, yell_2)));
                }
                else {
                    known_values.insert(oper_1.clone(), yell_1);
                    known_values.insert(oper_2.clone(), yell_2);
                }
            }
        }
        known_values.insert(id.to_owned(), old_yell);   
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    
    let mut known_values: HashMap<String, Yell> = HashMap::with_capacity(input.lines().count());

    for line in input.lines() {
        let (id, yell_str) = line.split_once(": ").unwrap();
        let id = String::from(id);
        known_values.insert(id, Yell::from_str(yell_str));
    }
    let monkey_ids = known_values.keys().cloned().collect::<Vec<_>>();

    loop {
        if known_values.get(&String::from("root")).unwrap().is_parent() { break }
        for id in monkey_ids.iter() {
            attach_branches(&mut known_values, id);
        }
    }
    let root = known_values.remove(&String::from("root")).unwrap();
    let part1 = root.value();
    println!("Part 1: {part1}, in {:#?}", now.elapsed());
}