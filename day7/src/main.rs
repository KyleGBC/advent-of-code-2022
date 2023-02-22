use std::str::Lines;

struct Node {
    pub children: Vec<Node>,
    pub name: String,
    pub size: Option<usize>,
}

impl Node {
    fn from_lines(lines: &mut Lines) -> Node {
        let mut new_node = Node{ children: Vec::new(), name: "/".to_string(), size: None};

        while let Some(line) = lines.next() {
            if line.starts_with('$') {
                let line = &line[2..];
                if let Some((cmd, name)) = line.split_once(' ') {
                    match(cmd, name) {
                        ("cd", "..") => return new_node,
                        ("cd", "/") => {},
                        ("cd", name) => {
                            let mut subnode = Node::from_lines(lines);
                            subnode.name = name.to_string();
                            new_node.children.push(subnode);

                        }
                        _ => {}
                    }   
                }
            } 
            else {
                let (first, name) = line.split_once(' ').unwrap();
                if let Ok(size) = first.parse::<usize>() {
                    new_node.children.push(Node{children: Vec::new(), name: name.to_string(), size: Some(size)})
                }
            }
        }
        new_node
    }

    fn size(&self, prefix: String, node_sizes: &mut Vec<usize>) -> usize {
        if let Some(s) = self.size {
            s
        }
        else {
            let s = self.children.iter().fold(0, |sum, n| sum + n.size(prefix.clone(), node_sizes));
            node_sizes.push(s);
            s
        }   
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input = std::fs::read_to_string("input.txt").expect("Couldn't open file");
    let mut lines = input.lines();

    let root = Node::from_lines(&mut lines);
    let mut dir_sizes = Vec::with_capacity(100);
    let used_space = root.size(String::from(" - "), &mut dir_sizes);

    let part1: usize = dir_sizes.iter().filter(|s| **s < 100000_usize).sum();
    let part2 = dir_sizes.iter().filter(|s| used_space - **s <= 40_000_000_usize).min().unwrap();

    println!("Part 1: {part1}, Part 2: {part2}, {} micros", now.elapsed().as_micros());
}
