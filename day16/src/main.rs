use std::collections::HashMap;
type ValveLabel = (usize, usize);
struct Valve { pub flow_rate: usize, pub tunnels: Vec<(usize, usize)> }

fn flow(all_valves: &HashMap<ValveLabel, Valve>, mut remaining_valves: Vec<ValveLabel>, distances: &HashMap<ValveLabel, HashMap<ValveLabel, usize>>, current: ValveLabel, time: usize) -> usize {    remaining_valves.swap_remove(remaining_valves.iter().position(|v| *v == current).unwrap());
    if time <= 2 {
         all_valves[&current].flow_rate * (time - 1)
    }
    else if let Some(d) = distances.get(&current) {
        d.iter().filter(|m| remaining_valves.contains(m.0) && all_valves[m.0].flow_rate > 0).map(|m| {
            let time_on_arrival = (time - 1).saturating_sub(*m.1);
            if time_on_arrival >= 1 {
                flow(all_valves, remaining_valves.clone(), distances, *m.0, time_on_arrival)
            }
            else { 0 }
        }).max().unwrap_or(0) + all_valves[&current].flow_rate * (time - 1)
    }
    else {
        let jammed_valve = &all_valves[&current];
        jammed_valve.tunnels.iter().filter(|t| remaining_valves.contains(t)).map(|t| flow(all_valves, remaining_valves.clone(), distances, *t, time - 1)).max().unwrap()
    }
}

fn dijkstra(grid: &HashMap<ValveLabel, Valve>, start: ValveLabel) -> HashMap<ValveLabel, usize> {
    let mut q: HashMap<ValveLabel, bool> = HashMap::new();
    let mut dis: HashMap<ValveLabel, usize> = HashMap::new();
    
    for k in grid.keys() {
        q.insert(*k, true);
        dis.insert(*k, usize::MAX);
    }
    *dis.get_mut(&start).unwrap() = 0;

    while q.values().any(|n| *n) {
        let cur_pos = *grid.keys().filter(|k| q[*k]).min_by(|a, b| dis[a].cmp(&dis[b])).unwrap();
        *q.get_mut(&cur_pos).unwrap() = false;

        let neighbors: Vec<ValveLabel> = grid[&cur_pos].tunnels.clone().into_iter().filter(|n| q[n]).collect();
        for v in neighbors {
            let alt= dis[&cur_pos] + 1;
            if alt < dis[&v] {
                *dis.get_mut(&v).unwrap() = alt;
            }
        }
    }
    dis
}

fn main() {
    let parse_time = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let mut valves: HashMap<(usize, usize), Valve> = HashMap::with_capacity(60);
    
    for line in input.lines() {
        let (valve, rest) = line.split_once(';').unwrap();
        let flow_rate = valve[valve.find('=').unwrap() + 1..].parse::<usize>().unwrap();
        let name = (valve.chars().nth(6).unwrap() as usize, valve.chars().nth(7).unwrap() as usize);
        let tunnels: Vec<(usize, usize)> = rest.split(|c: char| c.is_lowercase() || c == ',' || c == ' ').filter(|s| !s.is_empty()).map(|s| (s.chars().next().unwrap() as usize, s.chars().nth(1).unwrap() as usize) ).collect();
        valves.insert(name, Valve { flow_rate, tunnels } );
    }
    
    let mut distances: HashMap<ValveLabel, HashMap<ValveLabel, usize>> = HashMap::new();
    let non_zero_valves: Vec<ValveLabel> = valves.iter().filter(|(_, v)| v.flow_rate != 0).map(|(k, _)| *k).collect();
    for v in non_zero_valves {
        distances.insert(v, dijkstra(&valves, v));
    }


    let part1 = flow(&valves, valves.keys().copied().collect(), &distances, (65, 65), 30);
    println!("{part1}, in {:#?}", parse_time.elapsed());


    //get min distance between all non-zero valves to each other

}
