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
    geo_bots: i32,
    obi_bots: i32,
    clay_bots: i32,
    ore_bots: i32,
    geo: i32,
    obi: i32,
    clay: i32,
    ore: i32,
}
impl State {
    fn new(bp: Blueprint) -> Self {
        let mut ret = State::default();
        ret.blueprint = bp;
        ret.time = 24;
        ret.ore_bots = 1;
        ret
    }

    fn maximum_geodes(&self) -> i32 {
        //Hacky Depth first search with absolutely no optimizations
        return self.child_states().iter().map(|s| {
            if s.time == 0 { return self.geo }
            s.maximum_geodes()
        }).max().unwrap()
    }

    fn child_states(&self) -> [State; 4] {
        [self.construct_geo(), self.construct_obi(), self.construct_clay(), self.construct_ore()]
    }
    fn construct_ore(&self) -> State { 
        let time_taken = (self.blueprint.ore_ore_cost - self.ore) / self.ore_bots + if self.blueprint.ore_ore_cost % self.ore_bots > 0 { 2 } else { 1 };
        
        if time_taken >= self.time { return self.last_minutes() }

        let ore = self.ore + self.ore_bots * time_taken - self.blueprint.ore_ore_cost;
        let clay = self.clay + self.clay_bots * time_taken;
        let obi = self.obi + self.obi_bots * time_taken;
        let geo = self.geo + self.geo_bots * time_taken;
        
        State { ore, clay, obi, geo, time: self.time - time_taken, ore_bots: self.ore_bots + 1, ..(*self).clone() }
    }
    fn construct_clay(&self) -> State { 
        let time_taken = (self.blueprint.clay_ore_cost - self.ore) / self.ore_bots + if self.blueprint.clay_ore_cost % self.ore_bots > 0 { 2 } else { 1 };

        if time_taken >= self.time { return self.last_minutes() }

        let ore = self.ore + self.ore_bots * time_taken - self.blueprint.clay_ore_cost;
        let clay = self.clay + self.clay_bots * time_taken;
        let obi = self.obi + self.obi_bots * time_taken;
        let geo = self.geo + self.geo_bots * time_taken;
        
        State { ore, clay, obi, geo, time: self.time - time_taken, clay_bots: self.clay_bots + 1, ..(*self).clone() }
    }
    fn construct_obi(&self) -> State { 
        let time1 = (self.blueprint.obi_ore_cost - self.ore) / self.ore_bots + if self.blueprint.obi_ore_cost % self.ore_bots > 0 { 2 } else { 1 };
        let time2 = (self.blueprint.obi_clay_cost - self.clay) / self.clay_bots + if self.blueprint.obi_clay_cost % self.clay_bots > 0 { 2 } else { 1 };
        let time_taken = if time1 > time2 { time1 } else { time2 };

        if time_taken >= self.time { return self.last_minutes() }

        let ore = self.ore + self.ore_bots * time_taken - self.blueprint.obi_ore_cost;
        let clay = self.clay + self.clay * time_taken - self.blueprint.obi_clay_cost;
        let obi = self.obi + self.obi_bots * time_taken;
        let geo = self.geo + self.geo_bots * time_taken;
        
        State { ore, clay, obi, geo, time: self.time - time_taken, obi_bots: self.obi_bots + 1, ..(*self).clone() }
    }
    fn construct_geo(&self) -> State { 
        let time1 = (self.blueprint.geo_ore_cost - self.ore) / self.ore_bots + if self.blueprint.geo_ore_cost % self.ore_bots > 0 { 2 } else { 1 };
        let time2 = (self.blueprint.geo_obi_cost - self.obi) / self.obi_bots + if self.blueprint.geo_obi_cost % self.obi_bots > 0 { 2 } else { 1 };
        let time_taken = if time1 > time2 { time1 } else { time2 };

        if time_taken >= self.time { return self.last_minutes() }

        let ore = self.ore + self.ore_bots * time_taken - self.blueprint.geo_ore_cost;
        let clay = self.clay + self.clay_bots * time_taken;
        let obi = self.obi + self.obi_bots * time_taken - self.blueprint.geo_obi_cost;
        let geo = self.geo + self.geo_bots * time_taken;

        State { ore, clay, obi, geo, time: self.time - time_taken, geo_bots: self.geo_bots + 1, ..(*self).clone() }
    }
    fn last_minutes(&self) -> State {
        State { time: 0, geo: self.geo + self.geo_bots * self.time, ..(*self).clone() }
    }
   
}

fn main() {
    let now = std::time::Instant::now();
    let input = include_str!("../sample.txt");
    
    let part1 = input.lines().map(|l| {
        let bp = Blueprint::from_str(l);
        let initial = State::new(bp);
        initial.maximum_geodes()
    }).max().unwrap();

    println!("Part 1 is {}, in {:#?}", part1, now.elapsed());

}
