#![feature(drain_filter)]

use std::thread::{ScopedJoinHandle, self};
#[derive(Debug, Clone, Copy)]
struct Range {
    pub start: isize, 
    pub end: isize,
}
impl Range {
    fn intersecting(&self, other: &Range) -> bool {
        self.end >= other.start && other.end >= self.start
    }
    fn covering(&self) -> isize {
       self.end - self.start + 1 
    }
}
impl std::ops::Add<Range> for Range {
    type Output = Range;
    fn add(self, rhs: Range) -> Self::Output {
        Range{start: isize::min(self.start, rhs.start), end: isize::max(self.end, rhs.end)}
    }
}

fn covered_ranges_at_y(sensors_and_beacons: &Vec<((isize, isize), (isize, isize))>, y: isize) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::with_capacity(50);
    for (s, b) in sensors_and_beacons {
        let closest_distance = (s.0 - b.0).abs() + (s.1 - b.1).abs();
        let horizontal_range = closest_distance - (s.1 - y).abs();
        if horizontal_range >= 0 {
            let new_r = Range{start: s.0 - horizontal_range, end: s.0 + horizontal_range};
            let intersected_ranges = ranges.drain_filter(|r| r.intersecting(&new_r)).collect::<Vec<Range>>();
            let combined_range = intersected_ranges.iter().fold(new_r, |acc, i| acc + *i);
            ranges.push(combined_range);
        }
    }
    ranges
}

fn find_first_uncovered(ranges: &mut Vec<Range>, domain: std::ops::RangeInclusive<isize>) -> Option<isize> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    for range in ranges.iter() {
        if domain.contains(&(range.end + 1)) {
            return Some(range.end + 1)
        }
    }
    None
}

fn find_tuning_frequency_in_range(sensors_and_beacons: &Vec<((isize, isize), (isize, isize))>, domain: std::ops::RangeInclusive<isize>) -> Option<isize> {
    let part2_ranges = domain.map(|y| (y, covered_ranges_at_y(&sensors_and_beacons, y.clone()))).find(|(_, v)| v.len() > 1);
    if let Some((y, mut r)) = part2_ranges {
        return Some(find_first_uncovered(&mut r, 0..=4_000_000).unwrap() * 4_000_000 + y);
    }
    None
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let mut sensors_and_beacons: Vec<((isize, isize), (isize, isize))> = Vec::with_capacity(24);

    for line in input.lines() {
        let sb = match line.split([' ', ':', '=', ',']).collect::<Vec<_>>().as_slice() {
            &["Sensor", "at", "x", sx, "", "y", sy, "", "closest", "beacon", "is", "at", "x", bx, "", "y", by] => {
                ((sx.parse::<isize>().unwrap(), sy.parse::<isize>().unwrap()), (bx.parse::<isize>().unwrap(), by.parse::<isize>().unwrap()))
            }
            _ => panic!("Line couldn't be parsed as expected")
        };
        sensors_and_beacons.push(sb);
    }

    let part1 = covered_ranges_at_y(&sensors_and_beacons, 2_000_000).iter().map(|r| r.covering()).sum::<isize>() - 1;

    let mut part2 = 0;
    let nums: Vec<isize> = (0..=4_000_000).collect();
    thread::scope(|s| {
        let mut handles: Vec<ScopedJoinHandle<Option<isize>>> = Vec::with_capacity(12);
        for num_range in nums.chunks(33333) {
            handles.push(s.spawn(|| find_tuning_frequency_in_range(&sensors_and_beacons, num_range[0]..=num_range[num_range.len() - 1])));
        }
        
        for handle in handles {
            if let Some(freq) = handle.join().unwrap() {
                part2 = freq;
                break;
            }
        }
    });
    println!("Part 1: {part1}, Part2: {part2}, in {:#?}", now.elapsed());
}
