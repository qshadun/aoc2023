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
    let mut counts = vec![];
    // find the upstream module names of the conjuction module before rx
    for name in ["js", "qs", "dt", "ts"] {
        let mut machine = Machine::from_input(input);
        counts.push(machine.first_high(name));
    }
    let ans = lcm(&counts);
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
enum ModuleType<'a> {
    FlipFlop(FlipState),
    Conjunction(HashMap<&'a str, PulseType>),
    BroadCaster,
}

impl<'a> ModuleType<'a> {
    fn process(&mut self, from: &'a str, pt: PulseType) -> Option<PulseType> {
        match self {
            ModuleType::FlipFlop(state) => match pt {
                PulseType::Low => match state {
                    FlipState::Off => {
                        *state = FlipState::On;
                        Some(PulseType::High)
                    }
                    FlipState::On => {
                        *state = FlipState::Off;
                        Some(PulseType::Low)
                    }
                },
                PulseType::High => None,
            },
            ModuleType::Conjunction(remember) => {
                remember.insert(from, pt);
                let all_high = remember.values().all(|x| *x == PulseType::High);
                if all_high {
                    Some(PulseType::Low)
                } else {
                    Some(PulseType::High)
                }
            }
            ModuleType::BroadCaster => Some(pt),
        }
    }
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    module_type: ModuleType<'a>,
    downstream: Vec<&'a str>,
}

const BROADCASTER: &str = "broadcaster";
impl<'a> Module<'a> {
    fn from_line(line: &'a str) -> Self {
        let parts: Vec<_> = line.split(" -> ").collect();
        let mut name = parts[0];
        let downstream: Vec<_> = parts[1].split(", ").collect();
        let module_type = if name == BROADCASTER {
            ModuleType::BroadCaster
        } else if let Some(_name) = name.strip_prefix('&') {
            name = _name;
            ModuleType::Conjunction(HashMap::new())
        } else {
            name = &name[1..];
            ModuleType::FlipFlop(FlipState::Off)
        };
        Self {
            name,
            module_type,
            downstream,
        }
    }

    fn process(&mut self, from: &'a str, pt: PulseType) -> Vec<Pulse<'a>> {
        let mut ans = vec![];
        if let Some(next_pt) = self.module_type.process(from, pt) {
            for ds in self.downstream.iter() {
                ans.push(Pulse::new(self.name, ds, next_pt));
            }
        }
        ans
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
            modules.insert(m.name, m);
        }

        let up_to_down: HashMap<&str, Vec<&str>> = modules
            .iter()
            .map(|(k, v)| (*k, v.downstream.clone()))
            .collect();
        for (up, down) in up_to_down.into_iter() {
            for ds in down.into_iter() {
                if let Some(m) = modules.get_mut(ds) {
                    if let ModuleType::Conjunction(ref mut remember) = &mut m.module_type {
                        remember.insert(up, PulseType::Low);
                    }
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
