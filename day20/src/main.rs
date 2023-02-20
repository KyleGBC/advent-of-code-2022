use std::io::prelude::*;
#[derive(Clone, Debug)]
struct OrderedI32 { num: i32, index: i32 }

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let length = input.lines().count(); 
    
    let mut data: Vec<OrderedI32> = Vec::with_capacity(length);
    for (index, num) in input.lines().enumerate() {
        let num = num.parse::<i32>().unwrap();
        data.push(OrderedI32 { num, index: index as i32 })
    }

    for i in 0..length {
        let curr_idx = data[i].index;
        let j = i32::rem_euclid(curr_idx + data[i].num, length as i32);
        let k = i32::rem_euclid(curr_idx + data[i].num +  i32::signum(data[i].num), length as i32);
        let (l, u) = if j < k { (j, k) } else { (k, j) };
        let new_idx = if l != 0 {
            if curr_idx > u { l } else { u }
        }
        else {
            if data[i].num < 0 { u } else { l }
        };

        if new_idx < curr_idx {
            data.iter_mut().filter(|n| n.index < curr_idx && n.index >= new_idx).for_each(|n| n.index += 1);
            data[i].index = new_idx;
        }
        else {
            data.iter_mut().filter(|n| n.index > curr_idx && n.index <= new_idx).for_each(|n| n.index -= 1);
            data[i].index = new_idx;
        }



        let _ = std::io::stdin().read(&mut [0u8]);
    }
    let anchor = data.iter().find(|o| o.num == 0).unwrap().index;
    let part1: i32 = (1..=3).map(|n| data.iter().find(|o| o.index == (anchor + n*1000) % length as i32).unwrap().num).sum();
    println!("Part 1: {part1}, Part 2: , in {:#?}", now.elapsed());

}
