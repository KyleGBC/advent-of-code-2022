use std::{rc::Rc, cell::RefCell};
type NodeLink<'a> = Rc<RefCell<Node<'a>>>;

#[derive(Clone)]
struct Node<'a> {
    pub parent: Option<NodeLink<'a>>,
    pub children: Vec<NodeLink<'a>>,
    pub name: &'a str,
    pub size: Option<usize>,
}

impl<'a> Node<'a> {
    fn size(&self, mut prefix: String) -> usize {
        if let Some(s) = self.size {
            println!("{}{} with size {} in dir {}", prefix, self.name, s, self.parent.as_ref().unwrap().try_borrow().unwrap().name);
            s
        }
        else {
            println!("{}Dir {}:", prefix, self.name);
            let mut total = 0;
            prefix.push('\t');
            self.children.iter().for_each(|c| {
                total += c.try_borrow().unwrap().size(prefix.clone());
            });
            println!("{prefix}Totaling {total}");
            total
        }
        
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't open file");
    let root = Rc::new(RefCell::new(Node{parent: None, children: Vec::new(), name: "/", size: None}));
    let mut current_node: NodeLink = Rc::clone(&root);
    
    for line in input.lines() {
        if line.starts_with("$") {
            let line = &line[2..];
            if let Some((command, name)) = line.split_once(' ') {
                match command {
                    "cd" => match name {
                        "/" => current_node = Rc::clone(&root),
                        ".." => {
                            let parent = current_node.try_borrow().unwrap().clone().parent;
                            current_node = Rc::clone(&parent.expect("Current node shouldn't be root"));
                        }
                        _ => {
                            let children = current_node.try_borrow().unwrap().clone().children;
                            let l = children.iter().find(|c| c.try_borrow().unwrap().name == name).unwrap();
                            current_node = Rc::clone(l);
                        }
                    }
                    _ => unreachable!("There should always be a command after a $")
                }
            }
        }   
        else {
            let(first, name) = line.split_once(' ').unwrap();
            let size = if first == "dir" { None } else { Some(first.parse::<usize>().unwrap()) };

            let new_node = Node {parent: Some(Rc::clone(&current_node)), size, name, children: Vec::new()};
            current_node.borrow_mut().children.push(Rc::new(RefCell::new(new_node)));
        }
    }
    root.try_borrow().unwrap().size(String::new());
}
