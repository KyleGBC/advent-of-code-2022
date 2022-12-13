fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let mut history: Vec<i32> = Vec::with_capacity(250);

    let mut reg = 1;
    for line in input.lines() {
        match line.split_once(' ') {
            Some((_, val)) => {
                history.push(reg);
                history.push(reg);
                reg += val.parse::<i32>().unwrap();
            }, 
            None => history.push(reg),
        }
    }

    let part1: i32 = history.iter().enumerate().filter(|(c, _)| ((c + 1) % 40 == 20)).map(|(c, r)| (c + 1) as i32 * *r).sum();
    let part2: Vec<char> = history.iter().enumerate().map(|(c, r)| if ((c % 40) as i32 - (*r)).abs() < 2 {'█'} else {' '} ).collect();

    println!("Finished in {:#?}, now printing", now.elapsed());
    println!("{part1}");
    for line in part2.as_slice().chunks(40) {
        let p: String = line.iter().collect();
        println!("{p}");
    }
    println!("Overall took {:#?}", now.elapsed());
}
