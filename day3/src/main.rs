use std::{fs, collections::HashMap, time};

fn main() {
    let now = time::Instant::now();
    let mut c = 0;
    let priority_map: HashMap<char, i32> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().map(|letter| {c += 1; (letter, c)}).collect();

    let inventory = fs::read_to_string("input.txt").expect("Coud not read in file");
    let mut priority_total = 0_i32;
    let mut group_priority_total = 0_i32;
    let mut group_buf: Vec<&str> = Vec::with_capacity(3);

    for line in inventory.split("\r\n") {
        let (compartment1, compartment2) = line.split_at(line.len() / 2);

        for char in compartment1.chars() {
            if compartment2.contains(char) {
                priority_total += priority_map.get(&char).unwrap();
                break;
            }
        }
    
        group_buf.push(line);
        if group_buf.len() > 2 {
            let (line1, line2, line3) = match &group_buf[..] {&[a, b, c] => (a, b, c), _ => panic!()};
            for char in line1.chars() {
                if line2.contains(char) && line3.contains(char) {
                    group_priority_total += priority_map.get(&char).unwrap();
                    break;
                }
            } 
            group_buf.clear();  
        }
    }
    dbg!(priority_total);
    dbg!(group_priority_total);
    println!("finished in {} milliseconds", now.elapsed().as_micros() as f64 / 1000.0);    
}
