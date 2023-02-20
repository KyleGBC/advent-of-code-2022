//based on a solution by /u/SuperSmurfen
fn decrypt(nums: Vec<i128>, key: i128, times_mixed: i32) -> i128 {
    let nums = nums.into_iter().map(|n| n * key).collect::<Vec<_>>();
    let mut indices = (0..nums.len()).collect::<Vec<_>>();

    for _ in 0..times_mixed {
        for (i, &shift) in nums.iter().enumerate() {
            let curr_index = indices.iter().position(|n| *n == i).unwrap();
            indices.remove(curr_index);
            indices.insert(i128::rem_euclid(curr_index as i128 + shift, indices.len() as i128) as usize, i);
        }
    }
    let mixed = indices.iter().map(|&n| nums[n]).collect::<Vec<_>>();
    let anchor = mixed.iter().position(|&n| n == 0).unwrap();
    (1..=3).map(|n| mixed[(n*1000 + anchor).rem_euclid(mixed.len())]).sum()
}


fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let numbers = input.lines().map(|s| s.parse::<i128>().unwrap()).collect::<Vec<_>>();

    let part1 = decrypt(numbers.clone(), 1, 1);
    let part2 = decrypt(numbers, 811589153, 10);

    println!("Part 1: {part1}, Part 2: {part2}, in {:#?}", now.elapsed());
}
