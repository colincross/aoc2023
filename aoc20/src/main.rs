use std::collections::{BTreeMap, VecDeque};
use std::env;
use std::fs::read_to_string;

type Pulse = bool;

struct System {
    modules: BTreeMap<String, Module>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    inputs: BTreeMap<String, Pulse>,
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
        let output = (self.propagate)(&mut self.state, from, input);
        if output.is_some() {
            self.state.output = output.unwrap();
        }
        output
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
        Some(!state.output)
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
            .collect::<BTreeMap<_, _>>();

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

    fn pulse(&mut self) -> bool {
        let mut work_queue = VecDeque::<(String, String, Pulse)>::new();
        work_queue.push_back(("broadcaster".to_string(), "broadcaster".to_string(), false as Pulse));

        while let Some((name, from, pulse)) = work_queue.pop_front() {
            if name == "rx" && !pulse {
                return true;
            }

            if let Some(module) = self.modules.get_mut(name.as_str()) {
                if let Some(output_pulse) = module.input(from.as_str(), pulse) {
                    for output in &module.outputs {
                        //println!("{} {} {}", name, output_pulse, output);
                        work_queue.push_back((output.clone(), name.clone(), output_pulse))
                    }
                }
            }
        }
        return false;
    }

    fn state(&self) -> Vec<State> {
        let mut names = self.modules.keys().collect::<Vec<_>>();
        names.sort();
        names
            .iter()
            .map(|name| self.modules.get(name.as_str()).unwrap().state.clone())
            .collect()
    }

    fn prune(&mut self, start: &str) {
        let mut keeps = Vec::<&str>::new();
        keeps.push(start);
        let mut i = 0;
        while i < keeps.len() {
            let name = &keeps[i];
            let mut new_keeps = (&self.modules)
                .values()
                .filter(|module| module.outputs.contains(&name.to_string()))
                .map(|module| module.name.as_str())
                .filter(|new_keep| !keeps.contains(new_keep))
                .collect::<Vec<_>>();
            keeps.append(&mut new_keeps);
            i += 1;
        }

        dbg!(&keeps);

        let removes = (&self.modules)
            .keys()
            .filter(|key| !keeps.contains(&key.as_str()))
            .map(|s| s.clone())
            .collect::<Vec<_>>();

        dbg!(&removes);

        for remove in removes {
            self.modules.remove(&remove);
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let mut system = System::from(&data);

    let groups = vec![
        vec!["js", "dc", "dp", "xv", "rm", "hj", "bq", "gk", "hm", "rd", "xl", "gx"],
        vec!["ng", "vp", "vf", "th", "qb", "nc", "sd", "nl", "bt", "xd", "tn", "tv"],
        vec!["gr", "cb", "jg", "qn", "td", "zj", "vr", "hq", "kb", "mq", "fl", "gz"],
        vec!["lb", "nj", "xx", "hb", "qk", "hs", "fp", "xb", "tl", "kg", "px", "tm"],
    ];

    let summers = vec!["lr", "sn", "tf", "hl"];
    //system.prune("rx");

    // let mut old_states = BTreeMap::<Vec<State>, usize>::new();
    // for i in 0.. {
    //     println!("{}", i);
    //     let state = system.state();
    //     match old_states.entry(state) {
    //         Entry::Occupied(entry) => {
    //             println!("looped after {} iterations back to state {}", i, entry.get());
    //             break;
    //         }
    //         Entry::Vacant(entry) => entry.insert(i),
    //     };
    //     if system.pulse() {
    //         println!("done after {} iterations", i);
    //         break;
    //     }
    // }

    println!("{}", (0..)
        .map(|i| {
            println!("{} {} {}", i,
                     groups
                         .iter()
                         .map(|group| group_state(&system, &group))
                         .collect::<Vec<_>>()
                         .join(" "),
                     summers
                         .iter()
                         .map(|&name| system.modules[&name.to_string()].state.output.to_string())
                         .collect::<Vec<_>>()
                         .join(" "));

            //println!("{}", i);
            for (input, &value) in &system.modules["ql"].state.inputs {
                if value {
                    println!("{} {}", i, input);
                }
            }
            system.pulse()
        })
        .position(|pulse| pulse == true)
        .unwrap());
}

fn group_state(system: &System, group: &[&str]) -> String {
    group
        .iter()
        .rev()
        .map(|&name| if system.modules.get(&name.to_string()).unwrap().state.output { "1" } else { "0" })
        .collect::<Vec<_>>()
        .join("")
}