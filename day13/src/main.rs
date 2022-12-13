#[derive(PartialEq, Eq, Ord)]
enum List{ Parent(Vec<List>), Value(u32) }
impl List {
    fn from_str(s: &str) -> List {
        if s.starts_with('[') {
            let s = &s[1..s.len() - 1];
            let mut children: Vec<List> = Vec::new();
            let (mut depth, mut substring_start) = (0, 0);
            if s.len() == 0 {
                return List::Parent(children);
            }
            for (i, char) in s.chars().enumerate() {
                match char {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' if depth == 0 => {
                        children.push(List::from_str(&s[substring_start..i]));
                        substring_start = i + 1;
                    }
                    _ => {},
                }
            }
            children.push(List::from_str(&s[substring_start..]));
            List::Parent(children)
        }
        else { 
            List::Value(s.parse().unwrap())
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (List::Value(n), List::Value(other_n)) => n.partial_cmp(other_n),
            (List::Parent(children), List::Parent(other_children)) => children.partial_cmp(other_children),
            (List::Parent(_), List::Value(n))=> self.partial_cmp(&List::Parent(vec![List::Value(*n)])),
            (List::Value(n), List::Parent(other_children)) => vec![List::Value(*n)].partial_cmp(other_children),
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let mut packets: Vec<List> = input.split("\r\n").filter(|s| !s.is_empty()).map(|s| List::from_str(s)).collect();

    let part1 = packets.iter().as_slice().chunks(2).enumerate().fold(0, |sum, (i, pair)| if let [l1, l2] = pair {
        if l1 < l2 { sum + 1 + i } else { sum }
    } else { unreachable!("Packets should be in pairs")} );

    let part2 = {
        packets.push(List::from_str("[[2]]"));
        packets.push(List::from_str("[[6]]"));
        packets.sort_unstable();
        (packets.iter().position(|p| *p == List::from_str("[[2]]")).unwrap() + 1) * (packets.iter().position(|p| *p == List::from_str("[[6]]")).unwrap() + 1)
    };

    println!("Part 1: {part1}, Part 2: {part2}, in {:#?}", now.elapsed());
}
