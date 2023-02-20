fn ceil_div(a: i32, b: i32) -> i32 { if a <= 0 { return 0 } else { return a / b + if a % b > 0 { 1 } else { 0 } } }

#[derive(Debug, Default, Clone)]
struct Blueprint {
    id: i32,
    ore_ore_cost: i32,
    clay_ore_cost: i32,
    obi_ore_cost: i32,
    obi_clay_cost: i32,
    geo_ore_cost: i32,
    geo_obi_cost: i32
}
impl Blueprint {
    fn from_str(s: &str) -> Self {
        let s = s.split(|c| !char::is_numeric(c)).filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        match s.as_slice() {
            &[id, ore_ore_cost, clay_ore_cost, obi_ore_cost, obi_clay_cost, geo_ore_cost, geo_obi_cost] => Self { id, ore_ore_cost, clay_ore_cost, obi_ore_cost, obi_clay_cost, geo_ore_cost, geo_obi_cost },
            _ => panic!("Input line does not match expected format")
        }
    }
}

#[derive(Debug, Default, Clone)]
struct State {
    time: i32,
    blueprint: Blueprint,
    ore_bots: i32,
    clay_bots: i32,
    obi_bots: i32,
    geo_bots: i32,
    ore: i32,
    clay: i32,
    obi: i32,
    geo: i32,
}
impl State {
    fn new(bp: Blueprint, time: i32) -> Self {
        let mut ret = State::default();
        ret.blueprint = bp;
        ret.time = time;
        ret.ore_bots = 1;
        ret
    }
    fn maximum_geodes(&self) -> i32 {
        let mut max = 0;
        for s in self.child_states() {
            let upper_bound = s.geo + (s.time * s.geo_bots) + (s.time)*(s.time - 1)/2; 
            if max > upper_bound { continue };

            let geo = if s.time == 0 { s.geo } else { s.maximum_geodes() };
            max = max.max(geo)
        }
        max
    }
    fn child_states(&self) -> Vec<State> {
        let mut ret = Vec::with_capacity(4);

        if let Some(time) = self.geo_time() { ret.push(self.construct_geo(time)) }
        if let Some(time) = self.obi_time() { 
            if self.obi_bots < self.blueprint.geo_obi_cost {
                ret.push(self.construct_obi(time))
            }
        }
        if let Some(time) = self.clay_time() { 
            if self.clay_bots < self.blueprint.obi_clay_cost { 
                ret.push(self.construct_clay(time)) 
            } 
        }
        if let Some(time) = self.ore_time() {
            if self.ore_bots < (self.blueprint.ore_ore_cost.max(self.blueprint.clay_ore_cost).max(self.blueprint.geo_ore_cost).max(self.blueprint.obi_ore_cost)) {
                ret.push(self.construct_ore(time))
            }
        }



        if ret.is_empty() { ret.push(self.last_minutes()) }

        ret
    }
    fn ore_time(&self) -> Option<i32> {
        let time_taken = ceil_div(self.blueprint.ore_ore_cost - self.ore, self.ore_bots) + 1;
        return if time_taken >= self.time { Option::None } else { Option::Some(time_taken) }
    }
    fn construct_ore(&self, time_taken: i32) -> State { 
        let ore = self.ore + self.ore_bots * time_taken - self.blueprint.ore_ore_cost;
        let clay = self.clay + self.clay_bots * time_taken;
        let obi = self.obi + self.obi_bots * time_taken;
        let geo = self.geo + self.geo_bots * time_taken;
    
        State { ore, clay, obi, geo, time: self.time - time_taken, ore_bots: self.ore_bots + 1, ..(*self).clone() }
    }
    fn clay_time(&self) -> Option<i32> {
        let time_taken = ceil_div(self.blueprint.clay_ore_cost - self.ore,  self.ore_bots) + 1;
        return if time_taken >= self.time { Option::None } else { Option::Some(time_taken) }
    }
    fn construct_clay(&self, time_taken: i32) -> State { 
        let ore = self.ore + self.ore_bots * time_taken - self.blueprint.clay_ore_cost;
        let clay = self.clay + self.clay_bots * time_taken;
        let obi = self.obi + self.obi_bots * time_taken;
        let geo = self.geo + self.geo_bots * time_taken;
        
        State { ore, clay, obi, geo, time: self.time - time_taken, clay_bots: self.clay_bots + 1, ..(*self).clone() }
    }
    fn obi_time(&self) -> Option<i32> {
        if (self.blueprint.obi_clay_cost > self.clay) && self.clay_bots == 0 { return Option::None }

        let time1 = ceil_div(self.blueprint.obi_ore_cost - self.ore, self.ore_bots) + 1;
        let time2 = ceil_div(self.blueprint.obi_clay_cost - self.clay, self.clay_bots) + 1;
        let time_taken = i32::max(time1, time2);

        return if time_taken >= self.time { Option::None } else { Option::Some(time_taken) }
    }
    fn construct_obi(&self, time_taken: i32) -> State { 
        let ore = self.ore + self.ore_bots * time_taken - self.blueprint.obi_ore_cost;
        let clay = self.clay + self.clay_bots * time_taken - self.blueprint.obi_clay_cost;
        let obi = self.obi + self.obi_bots * time_taken;
        let geo = self.geo + self.geo_bots * time_taken;
        
        State { ore, clay, obi, geo, time: self.time - time_taken, obi_bots: self.obi_bots + 1, ..(*self).clone() }
    }
    fn geo_time(&self) -> Option<i32> {
        if (self.blueprint.geo_obi_cost > self.obi) && self.obi_bots == 0 { return Option::None }

        let time1 = ceil_div(self.blueprint.geo_ore_cost - self.ore, self.ore_bots) + 1;
        let time2 = ceil_div(self.blueprint.geo_obi_cost - self.obi, self.obi_bots) + 1;
        let time_taken = i32::max(time1, time2);

        return if time_taken >= self.time { Option::None } else { Option::Some(time_taken) }
    }
    fn construct_geo(&self, time_taken: i32) -> State { 
        let ore = self.ore + self.ore_bots * time_taken - self.blueprint.geo_ore_cost;
        let clay = self.clay + self.clay_bots * time_taken;
        let obi = self.obi + self.obi_bots * time_taken - self.blueprint.geo_obi_cost;
        let geo = self.geo + self.geo_bots * time_taken;

        State { ore, clay, obi, geo, time: self.time - time_taken, geo_bots: self.geo_bots + 1, ..(*self).clone() }
    }
    fn last_minutes(&self) -> State { State { time: 0, geo: self.geo + self.geo_bots * self.time, ..(*self).clone() } }
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../input.txt");
    
    let part1: i32 = input.lines().map(|l| {
        let bp = Blueprint::from_str(l);
        let initial = State::new(bp.clone(), 24);
        (initial.maximum_geodes(), bp.id)
    }).map(|(g, id)| g * id).sum();

    let part2 = input.lines().take(3).map(|l| {
        let bp = Blueprint::from_str(l);
        let initial = State::new(bp.clone(), 32);
        initial.maximum_geodes()
    }).fold(1, |acc, g| acc * g);

    println!("Part 1 is {}, Part 2 is {} in {:#?}", part1, part2, now.elapsed());
}
