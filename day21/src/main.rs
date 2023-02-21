use std::collections::HashMap;

#[derive(Clone)]
pub enum Op { Add, Sub, Mul, Div }
#[derive(Clone)]
pub struct YellOp { operation: Op, oper_1: String, oper_2: String }
#[derive(Clone)]
pub enum Yell { Imm(i128), Op(YellOp) }
impl Yell {
    fn from_string(s: &str) -> (String, Yell) {
        let (id, rem) = s.split_once(": ").unwrap();
        if rem.contains(|c| char::is_numeric(c)) {
            (String::from(id), Yell::Imm(rem.parse().unwrap()))
        }
        else {
            let mut op_string = rem.splitn(3, ' ');
            let oper_1 = op_string.next().unwrap().to_string();
            let operation = match op_string.next().unwrap() { "+" => Op::Add, "-" => Op::Sub, "*" => Op::Mul, "/" => Op::Div, _ => unreachable!("Not a valid math operation") };
            let oper_2 = op_string.next().unwrap().to_string();
            (String::from(id), Yell::Op(YellOp { operation, oper_1, oper_2 }))
        }
    }
    fn value(&self) -> Option<i128> { match self { Self::Imm(n) => Some(*n), Self::Op(_) => None }}
}

fn check_op_complete(monkeys: &mut HashMap<String, Yell>, monkey_id: &String, op: &YellOp) {
    let YellOp{operation, oper_1, oper_2, ..} = op;
    if let (Yell::Imm(a), Yell::Imm(b)) = (monkeys.get(oper_1).unwrap(), monkeys.get(oper_2).unwrap()) {
        let (a, b) = (*a, *b);
        let y = monkeys.get_mut(monkey_id).unwrap();
        *y = match operation {
            Op::Add => Yell::Imm(a + b),
            Op::Sub => Yell::Imm(a - b),
            Op::Mul => Yell::Imm(a * b),
            Op::Div => Yell::Imm(a / b)
        }  
    }
}
fn part1(input: &str) -> i128 {
    let mut monkeys: HashMap<String, Yell> = HashMap::with_capacity(input.lines().count());
    let mut monkey_names: Vec<String> = Vec::with_capacity(monkeys.len());

    for line in input.lines() {
        let (monkey_id, yell) = Yell::from_string(line);
        monkeys.insert(monkey_id.clone(), yell);
        monkey_names.push(monkey_id)
    }

    while let Yell::Op(_) = monkeys.get(&String::from("root")).unwrap() {
        for id in monkey_names.iter() {
            if let Yell::Op(op) = monkeys.get(id).unwrap().clone() {
                check_op_complete(&mut monkeys, &id, &op);
            }
        }
    }
    monkeys.get(&String::from("root")).unwrap().value().unwrap()
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    

    let part1 = part1(input.clone());
    println!("Part 1: {part1}, in {:#?}", now.elapsed());
}   
