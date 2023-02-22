
#[derive(Default, Debug, Clone)]
enum WorryOp{ Mul(u64), Add(u64), #[default] Square }

#[derive(Default, Debug, Clone)]
struct Monkey { pub items: Vec<u64>, pub op: WorryOp, pub div: u64, pub t_monkey: usize, pub f_monkey: usize, inspected: u32 }
impl Monkey{
    fn take_turn(&mut self, modulo: Option<u64>) -> Vec<(usize, u64)> {
        self.items.drain(0..).map(|mut i| {
            i = match self.op {
                WorryOp::Add(a) => i + a,
                WorryOp::Mul(m) => i * m,
                WorryOp::Square => i * i,
            };
            if let Some(m) = modulo {  i %= m } else { i /= 3 };
            self.inspected += 1;
            if i % self.div == 0 { (self.t_monkey, i ) } else { (self.f_monkey, i) }
        }).collect()
    }
}

fn simulate(mut monkeys: [Monkey; 8], rounds: u32, modulo: Option<u64>) -> u128 {
    for _ in 0..rounds {
        for id in 0..monkeys.len() {
            monkeys[id].take_turn(modulo).iter().for_each(|(id, item)| monkeys[*id].items.push(*item));
        }
    }
    monkeys.sort_by(|x, y| y.inspected.cmp(&x.inspected));
    (monkeys[0].inspected as u128) * (monkeys[1].inspected as u128)
}

fn main() {
    let input = include_str!("../input.txt");
    let mut monkeys: [Monkey; 8] = std::default::Default::default();

    let mut id = 0_usize;
    for line in input.lines() {
        match *line.trim().split([' ']).collect::<Vec<&str>>().as_slice() {
            ["Monkey", i] => id = i[..1].parse::<usize>().unwrap(),
            ["Test:", .., d] => monkeys[id].div = d.parse::<u64>().unwrap(),
            ["If", "true:", .., t] => monkeys[id].t_monkey = t.parse::<usize>().unwrap(),
            ["If", "false:", .., f] => monkeys[id].f_monkey = f.parse::<usize>().unwrap(),
            ["Operation:", .., "*", "old"] => monkeys[id].op = WorryOp::Square,
            ["Operation:", .., "*", n] => monkeys[id].op = WorryOp::Mul(n.parse::<u64>().unwrap()),
            ["Operation:", .., "+", n] => monkeys[id].op = WorryOp::Add(n.parse::<u64>().unwrap()),
            ["Starting", ..] => monkeys[id].items = line.split_once(": ").unwrap().1.split(", ").filter_map(|n| n.parse::<u64>().ok()).collect(),
            _ => {},
        }
    }

    let modulo = monkeys.iter().map(|m| m.div).product();
    let part1 = simulate(monkeys.clone(), 20, None);
    let part2 = simulate(monkeys, 10000, Some(modulo));
    println!("Part 1 {part1}, Part 2 {part2}");
}
