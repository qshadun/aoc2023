use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("inputs/input20.txt").unwrap();
    let mut machine = Machine::from_input(&input);
    machine.part1();
    part2(&input);
}

fn part2(input: &str) {
    let mut machine = Machine::from_input(input);
    let n1 = machine.first_high("js");
    let mut machine = Machine::from_input(input);
    let n2 = machine.first_high("qs");
    let mut machine = Machine::from_input(input);
    let n3 = machine.first_high("dt");
    let mut machine = Machine::from_input(input);
    let n4 = machine.first_high("ts");
    let ans = lcm(&[n1, n2, n3, n4]);
    println!("part2 = {ans}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug, Clone, Copy)]
struct Pulse<'a> {
    from: &'a str,
    dest: &'a str,
    pulse_type: PulseType,
}

impl<'a> Pulse<'a> {
    fn new(from: &'a str, dest: &'a str, pulse_type: PulseType) -> Self {
        Self {
            from,
            dest,
            pulse_type,
        }
    }
}

#[derive(Debug, Default)]
enum FlipState {
    #[default]
    Off,
    On,
}

#[derive(Debug)]
struct FlipFlop<'a> {
    name: &'a str,
    state: FlipState,
    downstream: Vec<&'a str>,
}

impl<'a> FlipFlop<'a> {
    fn new(name: &'a str, downstream: Vec<&'a str>) -> Self {
        Self {
            name,
            state: FlipState::Off,
            downstream,
        }
    }

    fn process(&mut self, pt: PulseType) -> Vec<Pulse<'a>> {
        let mut ans = vec![];
        match pt {
            PulseType::Low => match self.state {
                FlipState::Off => {
                    self.state = FlipState::On;
                    for ds in self.downstream.iter() {
                        ans.push(Pulse {
                            from: self.name,
                            dest: ds,
                            pulse_type: PulseType::High,
                        });
                    }
                }
                FlipState::On => {
                    self.state = FlipState::Off;
                    for ds in self.downstream.iter() {
                        ans.push(Pulse {
                            from: self.name,
                            dest: ds,
                            pulse_type: PulseType::Low,
                        });
                    }
                }
            },
            PulseType::High => {}
        };
        ans
    }
}

#[derive(Debug)]
struct Conjunction<'a> {
    name: &'a str,
    remember: HashMap<&'a str, PulseType>,
    downstream: Vec<&'a str>,
}

impl<'a> Conjunction<'a> {
    fn new(name: &'a str, downstream: Vec<&'a str>) -> Self {
        Self {
            name,
            remember: HashMap::new(),
            downstream,
        }
    }

    fn process(&mut self, from: &'a str, pt: PulseType) -> Vec<Pulse<'a>> {
        let mut ans = vec![];
        self.remember.insert(from, pt);
        let all_high = self.remember.values().all(|x| *x == PulseType::High);
        for ds in self.downstream.iter() {
            if all_high {
                ans.push(Pulse {
                    from: self.name,
                    dest: ds,
                    pulse_type: PulseType::Low,
                });
            } else {
                ans.push(Pulse {
                    from: self.name,
                    dest: ds,
                    pulse_type: PulseType::High,
                });
            }
        }
        ans
    }
}

#[derive(Debug)]
struct BroadCaster<'a> {
    name: &'a str,
    downstream: Vec<&'a str>,
}

impl<'a> BroadCaster<'a> {
    fn new(name: &'a str, downstream: Vec<&'a str>) -> Self {
        Self { name, downstream }
    }

    fn process(&self, pt: PulseType) -> Vec<Pulse<'a>> {
        let mut ans = vec![];
        for ds in self.downstream.iter() {
            ans.push(Pulse {
                from: self.name,
                dest: ds,
                pulse_type: pt,
            })
        }
        ans
    }
}

#[derive(Debug)]
enum Module<'a> {
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>),
    BroadCaster(BroadCaster<'a>),
}

const BROADCASTER: &str = "broadcaster";
impl<'a> Module<'a> {
    fn from_line(line: &'a str) -> Self {
        let parts: Vec<_> = line.split(" -> ").collect();
        let name = parts[0];
        let downstream: Vec<_> = parts[1].split(", ").collect();
        if name == BROADCASTER {
            Module::BroadCaster(BroadCaster::new(name, downstream))
        } else if let Some(name) = name.strip_prefix('&') {
            Module::Conjunction(Conjunction::new(name, downstream))
        } else {
            let name = &name[1..];
            Module::FlipFlop(FlipFlop::new(name, downstream))
        }
    }

    fn process(&mut self, from: &'a str, pt: PulseType) -> Vec<Pulse<'a>> {
        match self {
            Module::FlipFlop(inner) => inner.process(pt),
            Module::Conjunction(inner) => inner.process(from, pt),
            Module::BroadCaster(inner) => inner.process(pt),
        }
    }

    fn name(&self) -> &'a str {
        match self {
            Module::FlipFlop(inner) => inner.name,
            Module::Conjunction(inner) => inner.name,
            Module::BroadCaster(inner) => inner.name,
        }
    }

    fn downstream(&self) -> Vec<&'a str> {
        match self {
            Module::FlipFlop(inner) => inner.downstream.clone(),
            Module::Conjunction(inner) => inner.downstream.clone(),
            Module::BroadCaster(inner) => inner.downstream.clone(),
        }
    }
}

#[derive(Debug)]
struct Machine<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> Machine<'a> {
    fn from_input(input: &'a str) -> Self {
        let mut modules = HashMap::new();
        for line in input.lines() {
            let m = Module::from_line(line);
            let name = m.name();
            modules.insert(name, m);
        }

        let up_to_down: HashMap<&str, Vec<&str>> =
            modules.iter().map(|(k, v)| (*k, v.downstream())).collect();
        for (up, down) in up_to_down.into_iter() {
            for ds in down.into_iter() {
                if let Some(Module::Conjunction(m)) = modules.get_mut(ds) {
                    m.remember.insert(up, PulseType::Low);
                }
            }
        }
        Self { modules }
    }

    fn one_click(&mut self) -> (usize, usize) {
        let mut low = 0;
        let mut high = 0;
        let mut q = VecDeque::new();
        q.push_back(Pulse::new("button", BROADCASTER, PulseType::Low));
        while let Some(p) = q.pop_front() {
            let Pulse {
                from,
                dest,
                pulse_type,
            } = p;
            match pulse_type {
                PulseType::Low => low += 1,
                PulseType::High => high += 1,
            }
            if let Some(m) = self.modules.get_mut(dest) {
                for p in m.process(from, pulse_type) {
                    q.push_back(p);
                }
            }
        }
        (low, high)
    }

    fn part1(&mut self) {
        let mut total_low = 0;
        let mut total_high = 0;
        for _ in 0..1000 {
            let (low, high) = self.one_click();
            total_low += low;
            total_high += high;
        }
        println!(
            "part1 = {}, low = {}, high = {}",
            total_low * total_high,
            total_low,
            total_high
        );
    }

    fn one_click_first_high(&mut self, name: &str) -> bool {
        let mut q = VecDeque::new();
        q.push_back(Pulse::new("button", BROADCASTER, PulseType::Low));
        while let Some(p) = q.pop_front() {
            let Pulse {
                from,
                dest,
                pulse_type,
            } = p;
            if from == name && pulse_type == PulseType::High {
                return true;
            }
            if let Some(m) = self.modules.get_mut(dest) {
                for p in m.process(from, pulse_type) {
                    q.push_back(p);
                }
            }
        }
        false
    }

    fn first_high(&mut self, name: &str) -> usize {
        let mut cnt = 0;
        loop {
            cnt += 1;
            if self.one_click_first_high(name) {
                break;
            }
        }
        cnt
    }
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
