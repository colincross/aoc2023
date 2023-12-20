use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::read_to_string;

type Pulse = bool;

struct System {
    modules: HashMap<String, Module>,
}

#[derive(Default)]
struct State {
    inputs: HashMap<String, Pulse>,
    output: Pulse,
}

struct Module {
    name: String,
    outputs: Vec<String>,
    propagate: fn(state: &mut State, from: &str, input: Pulse) -> Option<Pulse>,
    state: State,
}

impl Module {
    fn input(&mut self, from: &str, input: Pulse) -> Option<Pulse> {
        (self.propagate)(&mut self.state, from, input)
    }

    fn connect(&mut self, from: &str) {
        self.state.inputs.insert(from.to_string(), false);
    }

    fn from(s: &str) -> Self {
        let (name, outputs) = s
            .split_once(" -> ").expect("arrow");

        let outputs = outputs
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let state = State::default();

        let first = name.chars().next().expect("first");

        let propagate = match first {
            'b' => broadcast,
            '%' => flipflop,
            '&' => conjunction,
            _ => panic!(),
        };

        let name = if first == 'b' {
            name
        } else {
            &name[1..]
        }.to_string();

        Self {
            name,
            outputs,
            propagate,
            state,
        }
    }
}

fn broadcast<'a>(_state: &mut State, _from: &str, input: Pulse) -> Option<Pulse> {
    Some(input)
}

fn flipflop<'a>(state: &mut State, _from: &str, input: Pulse) -> Option<Pulse> {
    if !input {
        state.output = !state.output;
        Some(state.output)
    } else {
        None
    }
}

fn conjunction<'a>(state: &mut State, from: &str, input: Pulse) -> Option<Pulse> {
    *state.inputs.get_mut(from).unwrap() = input;
    let output = !state.inputs.values().all(|&v| v == true);
    Some(output)
}


impl System {
    fn from(data: &str) -> Self {
        let mut modules = data
            .lines()
            .map(Module::from)
            .map(|module| (module.name.clone(), module))
            .collect::<HashMap<_, _>>();

        let connections = modules
            .values()
            .map(|module| (module.name.clone(), module.outputs.clone()))
            .collect::<Vec<_>>();

        for (name, outputs) in &connections {
            for output in outputs {
                if let Some(module) = modules.get_mut(output.as_str()) {
                    module.connect(name);
                }
            }
        }

        Self { modules }
    }

    fn pulse(&mut self) -> (usize, usize) {
        let mut work_queue = VecDeque::<(String, String, Pulse)>::new();
        let mut low = 0;
        let mut high = 0;
        work_queue.push_back(("broadcaster".to_string(), "broadcaster".to_string(), false as Pulse));

        while let Some((name, from, pulse)) = work_queue.pop_front() {
            if pulse {
                high += 1;
            } else {
                low += 1;
            }

            if let Some(module) = self.modules.get_mut(name.as_str()) {
                if let Some(output_pulse) = module.input(from.as_str(), pulse) {
                    for output in &module.outputs {
                        work_queue.push_back((output.clone(), name.clone(), output_pulse))
                    }
                }
            }
        }
        (low, high)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let mut system = System::from(&data);

    let (low, high) = (0..1000)
        .map(|_| system.pulse())
        .fold((0, 0),
              |acc, (low, high)| (acc.0 + low, acc.1 + high));
    println!("{} {} {}", low, high, low * high);
}
