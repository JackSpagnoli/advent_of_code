use std::{
    collections::{HashMap, VecDeque, HashSet},
    vec,
};

use itertools::Itertools;
use num::integer::lcm;

pub mod task1 {
    use super::count_pulses;

    pub fn ans() -> u128 {
        count_pulses("resources/2023/day20/input")
    }
}

pub mod task2 {
    use super::single_low_rx_pulse;

    pub fn ans() -> u128 {
        single_low_rx_pulse("resources/2023/day20/input")
    }
}

fn count_pulses(file: &str) -> u128 {
    let mut modules = parse_file(file);

    let mut high = 0;
    let mut low = 0;

    let mut high_pulses = 0;
    let mut low_pulses = 0;

    (0..1000).for_each(|_| {
        ((high, low), modules, _) = push_button(modules.clone());

        high_pulses += high;
        low_pulses += low;
    });

    high_pulses * low_pulses
}

fn single_low_rx_pulse(file: &str) -> u128 {
    // I based this solution off a peak on Reddit. There was a lot of talk of subnets, lowest
    // common multiples, and NAND gates. There were also a few graphviz diagrams.
    // Between these hints I worked back to a solution I'll describe below.

    // All inputs for today's problem have the same structure, so
    // while this isn't general for all networks, it is for all of today's inputs.

    // The network is split into 4 subnets, each fed by one of broadcaster's outputs,
    // and each having an output node. The 4 output nodes feed into an inverter node, which feed into an rx feeder.
    // In the case of my network rx is fed from gf, the 4 inverter nodes are kr, zs, kf, and qk,
    // and these are fed by the output nodes bf, cx, gm, and qr respectively.
    // The 4 subnets are fed by broadcaster nodes kg, dz, ff, and bq respectively.
    // Each subnet's input node connects to the respective output node.

    // Below is a link to a graphviz side to illustrate what I mean.
    // https://dreampuf.github.io/GraphvizOnline/#digraph%20G%20%7B%0A%22mh%22%20-%3E%20rz%3B%0A%22nd%22%20-%3E%20jx%3B%0A%22xt%22%20-%3E%20cx%3B%0A%22dp%22%20-%3E%20mh%3B%0A%22pz%22%20-%3E%20zg%2C%20bf%3B%0A%22rp%22%20-%3E%20jb%2C%20bf%3B%0A%22jb%22%20-%3E%20bf%2C%20kp%3B%0A%22rj%22%20-%3E%20xt%2C%20cx%3B%0A%22hg%22%20-%3E%20dl%2C%20bf%3B%0A%22pt%22%20-%3E%20gm%2C%20vv%3B%0A%22pf%22%20-%3E%20xk%2C%20qr%3B%0A%22cv%22%20-%3E%20jp%2C%20cx%3B%0A%22zg%22%20-%3E%20bb%3B%0A%22qn%22%20-%3E%20gm%2C%20bh%3B%0A%22kp%22%20-%3E%20pz%3B%0A%22kg%22%20-%3E%20gm%2C%20pt%3B%0A%22sl%22%20-%3E%20rp%3B%0A%22dz%22%20-%3E%20bf%2C%20dc%3B%0A%22hm%22%20-%3E%20cx%2C%20tz%3B%0A%22dc%22%20-%3E%20fk%3B%0A%22xk%22%20-%3E%20qr%2C%20sf%3B%0A%22kr%22%20-%3E%20gf%3B%0A%22bq%22%20-%3E%20qr%2C%20mg%3B%0A%22sf%22%20-%3E%20qr%3B%0A%22cx%22%20-%3E%20ff%2C%20vx%2C%20zs%3B%0A%22hr%22%20-%3E%20fq%2C%20gm%3B%0A%22ls%22%20-%3E%20lf%2C%20gm%3B%0A%22mf%22%20-%3E%20cx%2C%20sx%3B%0A%22vq%22%20-%3E%20gm%3B%0A%22sx%22%20-%3E%20cx%2C%20rj%3B%0A%22gm%22%20-%3E%20kg%2C%20kf%2C%20fq%2C%20nc%2C%20lf%3B%0A%22jx%22%20-%3E%20qr%2C%20zz%3B%0A%22tz%22%20-%3E%20mf%2C%20cx%3B%0A%22jp%22%20-%3E%20cx%2C%20kt%3B%0A%22bb%22%20-%3E%20hg%2C%20bf%3B%0A%22zz%22%20-%3E%20pf%2C%20qr%3B%0A%22qr%22%20-%3E%20dp%2C%20bq%2C%20nd%2C%20rz%2C%20mg%2C%20qk%2C%20mh%3B%0A%22nc%22%20-%3E%20gb%3B%0A%22kt%22%20-%3E%20hm%2C%20cx%3B%0A%22mg%22%20-%3E%20dp%3B%0A%22dl%22%20-%3E%20bf%3B%0A%22zs%22%20-%3E%20gf%3B%0A%22bf%22%20-%3E%20dz%2C%20zg%2C%20kr%2C%20sl%2C%20fk%2C%20kp%2C%20dc%3B%0A%22bh%22%20-%3E%20vq%2C%20gm%3B%0A%22kf%22%20-%3E%20gf%3B%0A%22fq%22%20-%3E%20qn%3B%0A%22vl%22%20-%3E%20vx%2C%20cx%3B%0A%22qk%22%20-%3E%20gf%3B%0A%22fk%22%20-%3E%20sl%3B%0A%22tj%22%20-%3E%20nd%2C%20qr%3B%0A%22gb%22%20-%3E%20ls%2C%20gm%3B%0A%22lf%22%20-%3E%20hr%3B%0A%22vx%22%20-%3E%20cv%3B%0A%22ff%22%20-%3E%20vl%2C%20cx%3B%0A%22broadcaster%20%22-%3E%20kg%2C%20dz%2C%20ff%2C%20bq%3B%0A%22vv%22%20-%3E%20nc%2C%20gm%3B%0A%22gf%22%20-%3E%20rx%3B%0A%22rz%22%20-%3E%20tj%3B%0A%7D

    // The idea behind this solution is that the 4 inverter nodes together with the rx feeder form a NAND gate
    // between the 4 subnets. Ie. the rx feeder will only pulse when all 4 subnet outputs are low.

    // Since the network starts with all nodes low, we can consider each subnet independently. By finding their
    // cycle periods, we can use the lowest common multiple of these to get the period
    // of the NAND gate.

    // This solution starts by marking the rx_feeder, the nand gate nodes, and the nand gate inputs.
    // Then, by working from each broadcase node, the subnets are divided, and the nand gate inputs are assigned.
    // Each subnet then has it's period calculated, and the lowest common multiple is returned.
    let modules = parse_file(file);

    let (_rx_feeder, nand_gate_modules, _nand_gate_inputs, subnet_inputs) =
        find_critical_modules(&modules);

    let periods: Vec<u128> = nand_gate_modules
        .into_iter()
        .zip(subnet_inputs)
        .map(|(nand_module, subnet_input)| {
            find_subnet(&modules, subnet_input, nand_module)
        })
        .map(|subnet| {
            find_output_period(subnet)
        }).collect();

    periods.into_iter().fold(1, lcm)
}

fn find_critical_modules(modules: &ModuleMap) -> (String, Vec<String>, Vec<String>, Vec<String>) {
    let rx_feeder = modules
        .iter()
        .find(|(_, module)| get_connections(module).contains(&"rx".to_string()))
        .unwrap()
        .0
        .clone();

    let nand_gate_modules = modules
        .iter()
        .filter(|(_, module)| get_connections(module).contains(&rx_feeder))
        .map(|(name, _)| name.clone())
        .collect::<Vec<String>>();

    let nand_gate_inputs = nand_gate_modules
        .iter()
        .map(|nand_module| {
            modules
                .iter()
                .find(|(_, module)| get_connections(module).contains(nand_module))
                .unwrap()
                .0
                .clone()
        })
        .collect::<Vec<String>>();

    let subnet_inputs = nand_gate_inputs
        .iter()
        .map(|subnet_output| {
            modules
                .iter()
                .filter(|(_, module)| get_connections(module).contains(subnet_output))
                .filter(|(module_name, _)| {
                    get_connections(modules.get(&"broadcaster".to_string()).unwrap())
                        .contains(module_name)
                })
                .map(|(name, _)| name.clone())
                .next()
                .unwrap()
        })
        .collect::<Vec<String>>();

    (
        rx_feeder,
        nand_gate_modules,
        nand_gate_inputs,
        subnet_inputs,
    )
}

fn find_subnet(modules: &ModuleMap, subnet_input: String, subnet_output: String) -> ModuleMap{
    let mut subnet_set = HashSet::new();
    subnet_set.insert(subnet_input.clone());

    loop{
        let new_connections = find_new_connections(modules, &subnet_set, subnet_output.clone());

        if new_connections.is_empty() {
            break;
        }

        subnet_set = subnet_set.union(&new_connections).cloned().collect();
    }

    subnet_set.insert(subnet_output.clone());

    let mut subnet: ModuleMap = modules
        .iter()
        .filter(|(name, _)| subnet_set.contains(*name))
        .map(|(name, module)| (name.clone(), module.clone()))
        .collect();
    subnet.insert("broadcaster".to_string(), Module::Broadcast(vec![subnet_input.clone()]));

    subnet
}

fn find_new_connections(modules: &ModuleMap, subnet_set: &HashSet<String>, subnet_output: String) -> HashSet<String> {
    let mut new_connections = HashSet::new();

    subnet_set.iter().for_each(|subnet_module| {
        let module = modules.get(subnet_module).unwrap();
        let connections = get_connections(module);

        connections.iter().for_each(|connection| {
            if !subnet_set.contains(connection) && connection != &subnet_output {
                new_connections.insert(connection.clone());
            }
        });
    });

    new_connections
}

fn find_output_period(mut modules: ModuleMap) -> u128 {
    let mut state_memory: HashMap<String, u128> = HashMap::new();

    let mut period = 0;

    loop{
        let subnet_hash = hash_modules(&modules);

        if state_memory.contains_key(&subnet_hash) {
            return period - state_memory.get(&subnet_hash).unwrap();
        }

        state_memory.insert(subnet_hash, period);

        (_, modules, _) = push_button(modules);

        period += 1;
    }
}

fn hash_modules(modules: &ModuleMap) -> String {
    // Hashes a subnet into a string
    let mut mod_states = modules.iter().map(|(name, module)| {
        let module_state = match module {
            Module::Broadcast(_) => "0".to_string(),
            Module::FlipFlop(_, state) => match state {
                State::High => "1".to_string(),
                State::Low => "2".to_string(),
            },
            Module::Conjunction(_, feeders) => {
                let feeder_states = feeders.iter().map(|(_, state)| match state {
                    State::High => "1",
                    State::Low => "2",
                }).collect::<Vec<&str>>();

                feeder_states.into_iter().fold("".to_string(), |acc, state| format!("{}{}", acc, state)).to_string()
            }
        };
        format!("{}{}", name, module_state)
    }).collect::<Vec<String>>();
    mod_states.sort();
    mod_states.join("").to_string()
}

// (To, From, State)
type Pulse = (ModuleName, ModuleName, State);
type RxPresses = usize;
fn push_button(mut modules: ModuleMap) -> ((u128, u128), ModuleMap, RxPresses) {
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();

    pulse_queue.push_back(("broadcaster".to_string(), "button".to_string(), State::Low));

    let mut high_pulses = 0;
    let mut low_pulses = 0;

    let mut low_rx_pulses = 0;

    while let Some((module_to_name, module_from_name, pulse_state)) = pulse_queue.pop_front() {
        match pulse_state {
            State::High => high_pulses += 1,
            State::Low => low_pulses += 1,
        }

        if module_to_name == "rx" && pulse_state == State::Low {
            low_rx_pulses += 1;
        }

        if !modules.contains_key(&module_to_name) {
            continue;
        }

        let module = modules.remove(&module_to_name).unwrap();

        match module {
            Module::Broadcast(connections) => {
                connections
                    .clone()
                    .into_iter()
                    .map(|module_name| {
                        (
                            module_name.clone(),
                            module_to_name.clone(),
                            pulse_state.clone(),
                        )
                    })
                    .for_each(|pulse| pulse_queue.push_back(pulse));
                let module = Module::Broadcast(connections);
                modules.insert(module_to_name, module);
            }
            Module::FlipFlop(..) => {
                let (module, pulses) = pulse_flip_flop(module_to_name.clone(), module, pulse_state);
                modules.insert(module_to_name, module);
                pulses
                    .into_iter()
                    .for_each(|pulse| pulse_queue.push_back(pulse));
            }
            Module::Conjunction(..) => {
                let (module, pulses) = pulse_conjunction(
                    module_to_name.clone(),
                    module,
                    module_from_name,
                    pulse_state,
                );
                modules.insert(module_to_name, module);
                pulses
                    .into_iter()
                    .for_each(|pulse| pulse_queue.push_back(pulse));
            }
        }
    }

    ((high_pulses, low_pulses), modules, low_rx_pulses)
}

fn pulse_flip_flop(module_name: ModuleName, module: Module, pulse: State) -> (Module, Vec<Pulse>) {
    if pulse == State::High {
        return (module, vec![]);
    }

    let (connections, current_state) = match module {
        Module::FlipFlop(connections, current_state) => (connections, current_state),
        _ => panic!("bruh"),
    };

    if current_state == State::Low {
        let pulses = connections
            .clone()
            .into_iter()
            .map(|conn| (conn, module_name.clone(), State::High))
            .collect();

        let module = Module::FlipFlop(connections, State::High);
        return (module, pulses);
    }

    let pulses = connections
        .clone()
        .into_iter()
        .map(|conn| (conn, module_name.clone(), State::Low))
        .collect();
    let module = Module::FlipFlop(connections, State::Low);

    (module, pulses)
}

fn pulse_conjunction(
    module_name: ModuleName,
    module: Module,
    pulse_origin: ModuleName,
    pulse: State,
) -> (Module, Vec<Pulse>) {
    let (connections, mut feeders) = match module {
        Module::Conjunction(c, f) => (c, f),
        _ => panic!("bruh"),
    };

    let origin_index = feeders
        .iter()
        .find_position(|(name, _)| name == &pulse_origin)
        .unwrap()
        .0;

    feeders[origin_index] = (pulse_origin, pulse);

    if feeders.iter().all(|(_, state)| state == &State::High) {
        let pulses = connections
            .clone()
            .into_iter()
            .map(|conn| (conn, module_name.clone(), State::Low))
            .collect();

        let module = Module::Conjunction(connections, feeders);

        (module, pulses)
    } else {
        let pulses = connections
            .clone()
            .into_iter()
            .map(|conn| (conn, module_name.clone(), State::High))
            .collect();

        let module = Module::Conjunction(connections, feeders);

        (module, pulses)
    }
}

type ModuleMap = HashMap<ModuleName, Module>;
type ModuleName = String;
#[derive(PartialEq, Debug, Clone)]
enum State {
    High,
    Low,
}
#[derive(PartialEq, Debug, Clone)]
enum Module {
    Broadcast(Vec<ModuleName>),
    FlipFlop(Vec<ModuleName>, State),
    Conjunction(Vec<ModuleName>, Vec<(ModuleName, State)>),
}
fn parse_file(file: &str) -> ModuleMap {
    let contents = std::fs::read_to_string(file).unwrap();

    let modules = contents.lines().map(parse_line).collect();

    populate_conjunction_feeders(modules)
}

fn parse_line(line: &str) -> (ModuleName, Module) {
    let mut parts = line.split(" -> ");
    let module_name = parts.next().unwrap();
    let module_connections = parts.next().unwrap();

    let connections = module_connections
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    match module_name.chars().next().unwrap() {
        'b' => (module_name.to_string(), Module::Broadcast(connections)),
        '%' => (
            module_name[1..].to_string(),
            Module::FlipFlop(connections, State::Low),
        ),
        '&' => (
            module_name[1..].to_string(),
            Module::Conjunction(connections, vec![]),
        ),
        _ => panic!("bruh"),
    }
}

fn populate_conjunction_feeders(mut modules: ModuleMap) -> ModuleMap {
    let conjunctions: Vec<String> = modules
        .iter()
        .filter(|(_, module)| is_conjunction(module))
        .map(|(name, _)| name.clone())
        .collect();

    conjunctions.into_iter().for_each(|module_name| {
        let module = modules.remove(&module_name).unwrap();
        let connections = get_connections(&module);

        let feeders: Vec<(ModuleName, State)> = modules
            .iter()
            .filter(|(_, module)| get_connections(module).contains(&module_name))
            .map(|(name, _)| name.clone())
            .map(|module_name| (module_name, State::Low))
            .collect();

        let module = Module::Conjunction(connections.clone(), feeders);
        modules.insert(module_name, module);
    });

    modules
}

fn is_conjunction(module: &&Module) -> bool {
    matches!(module, Module::Conjunction(..))
}

fn get_connections(module: &Module) -> &Vec<ModuleName> {
    match module {
        Module::Broadcast(connections)
        | Module::FlipFlop(connections, _)
        | Module::Conjunction(connections, _) => connections,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_modules() {
        let file = "resources/2023/day20/test_input";

        let mut expected_modules = HashMap::new();

        expected_modules.insert(
            "broadcaster".to_string(),
            Module::Broadcast(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
        );
        expected_modules.insert(
            "a".to_string(),
            Module::FlipFlop(vec!["b".to_string()], State::Low),
        );
        expected_modules.insert(
            "b".to_string(),
            Module::FlipFlop(vec!["c".to_string()], State::Low),
        );
        expected_modules.insert(
            "c".to_string(),
            Module::FlipFlop(vec!["inv".to_string()], State::Low),
        );
        expected_modules.insert(
            "inv".to_string(),
            Module::Conjunction(vec!["a".to_string()], vec![("c".to_string(), State::Low)]),
        );

        let modules = parse_file(file);

        for (module_name, module) in modules {
            assert_eq!(expected_modules.get(&module_name).unwrap(), &module);
        }
    }

    #[test]
    fn test_push_button() {
        let modules = parse_file("resources/2023/day20/test_input");

        let ((high, low), _, _) = push_button(modules);

        assert_eq!(high, 4);
        assert_eq!(low, 8);
    }

    #[test]
    fn test_count_pulses() {
        assert_eq!(count_pulses("resources/2023/day20/test_input"), 32000000);
    }

    #[test]
    fn test_count_pulses2() {
        assert_eq!(count_pulses("resources/2023/day20/test_input2"), 11687500);
    }

    #[test]
    fn test_find_critical_modules() {
        let modules = parse_file("resources/2023/day20/input");

        let (rx_feeder, nand_gate_modules, nand_gate_inputs, subnet_inputs) =
            find_critical_modules(&modules);

        assert_eq!(rx_feeder, "gf");

        let expected_nand_gate_modules = ["kr", "zs", "kf", "qk"];
        assert_eq!(nand_gate_modules.len(), expected_nand_gate_modules.len());
        assert!(expected_nand_gate_modules
            .iter()
            .all(|module| nand_gate_modules.contains(&module.to_string())));

        let expected_nand_gate_inputs = ["bf", "cx", "gm", "qr"];
        assert_eq!(nand_gate_inputs.len(), expected_nand_gate_inputs.len());
        assert!(expected_nand_gate_inputs
            .iter()
            .all(|input| nand_gate_inputs.contains(&input.to_string())));

        let expected_subnet_inputs = ["kg", "dz", "ff", "bq"];
        assert_eq!(subnet_inputs.len(), expected_subnet_inputs.len());
        assert!(expected_subnet_inputs
            .iter()
            .all(|input| subnet_inputs.contains(&input.to_string())));
    }

    #[test]
    fn test_find_subnet(){
        let modules = parse_file("resources/2023/day20/input");

        let subnet = find_subnet(&modules, "dz".to_string(), "kr".to_string());

        let expected_modules = ["dz", "kr", "broadcaster", "dc", "fk", "bf", "sl", "dl", "hg", "rp", "jb", "bb", "zg", "kp", "pz"];
        assert_eq!(subnet.len(), expected_modules.len());
        assert!(expected_modules
            .iter()
            .all(|module| subnet.contains_key(&module.to_string())));
    }
}

