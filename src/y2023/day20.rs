use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug, PartialEq, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Clone)]
enum ModuleType {
    FlipFlop(FlipFlopState),
    Conjunction { history: HashMap<String, Pulse> },
    Broadcast,
    TestType,
}

#[derive(Debug, PartialEq, Clone)]
struct Module {
    name: String,
    module_type: ModuleType,
    connections: Vec<String>,
    output: Option<Pulse>,
}

impl Module {
    fn handle_input(&mut self, pulse: &Pulse, source: String) {
        match (pulse, self.module_type.clone()) {
            (_, ModuleType::Broadcast | ModuleType::TestType) => self.output = Some(pulse.clone()),
            (Pulse::High, ModuleType::FlipFlop(_)) => self.output = Some(Pulse::High),
            (Pulse::Low, ModuleType::FlipFlop(FlipFlopState::On)) => {
                self.module_type = ModuleType::FlipFlop(FlipFlopState::Off);
                self.output = Some(Pulse::Low);
            }
            (Pulse::Low, ModuleType::FlipFlop(FlipFlopState::Off)) => {
                self.module_type = ModuleType::FlipFlop(FlipFlopState::On);
                self.output = Some(Pulse::High);
            }
            (_, ModuleType::Conjunction { mut history }) => {
                history.insert(source, pulse.clone());
                self.module_type = ModuleType::Conjunction {
                    history: history.clone(),
                };
                if history.values().all(|x| *x == Pulse::High) {
                    self.output = Some(Pulse::Low);
                } else {
                    self.output = Some(Pulse::High);
                }
            }
        }
    }
}

fn trim_name(name: &str) -> String {
    if name.starts_with('&') || name.starts_with('%') {
        return name[1..].to_string();
    }
    name.to_string()
}

fn is_everything_off(modules: &[Module]) -> bool {
    let mut result = Vec::new();
    for module in modules {
        match &module.module_type {
            ModuleType::Conjunction { history } => {
                let turned_off = history.clone().values().all(|x| *x == Pulse::Low);
                result.push((module.name.clone(), turned_off));
            }
            ModuleType::FlipFlop(state) => {
                let turned_off = *state == FlipFlopState::Off;
                result.push((module.name.clone(), turned_off));
            }
            _ => {}
        }
    }

    result.iter().all(|(_, b)| *b)
}

fn process(name: &str, pulse: &Pulse, mut modules: Vec<Module>) -> (usize, usize, usize) {
    let mut low = 0;
    let mut high = 0;

    // For test_case 2 in part 1
    modules.push(Module {
        name: "output".to_string(),
        module_type: ModuleType::TestType,
        connections: Vec::new(),
        output: None,
    });
    // For real input
    modules.push(Module {
        name: "rx".to_string(),
        module_type: ModuleType::TestType,
        connections: Vec::new(),
        output: None,
    });

    let mut button_pushes = 0;

    while button_pushes < 1000 && (button_pushes == 0 || !is_everything_off(&modules)) {
        let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();
        queue.push_back((name.to_string(), pulse.clone(), "button".to_string()));

        while !queue.is_empty() {
            let (state, input, source) = queue.pop_front().unwrap();
            let mut current_state = modules
                .iter_mut()
                .find(|x| x.name == state)
                .unwrap()
                .clone();

            if input == Pulse::High {
                high += 1;
            } else {
                low += 1;
            }

            if std::mem::discriminant(&current_state.module_type)
                == std::mem::discriminant(&ModuleType::FlipFlop(FlipFlopState::On))
                && input == Pulse::High
            {
                continue;
            }

            current_state.handle_input(&input, source);
            for connection in current_state.connections.clone() {
                queue.push_back((
                    connection.to_string(),
                    current_state.output.as_ref().unwrap().clone(),
                    current_state.name.clone(),
                ));
            }
            modules = modules
                .clone()
                .into_iter()
                .map(|x| {
                    if x.name == current_state.name {
                        current_state.clone()
                    } else {
                        x
                    }
                })
                .collect();
        }

        button_pushes += 1;
    }

    (button_pushes, low, high)
}

fn find_sources(name: &str, input: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in input.lines() {
        let mut split = line.split("->");
        let lhs = split.next().unwrap();
        let rhs = split.next().unwrap();
        if rhs.contains(name) {
            result.push(trim_name(lhs.trim()));
        }
    }

    result
}

fn parse_line(line: &str, input: &str) -> Module {
    let mut split = line.trim().split("->");
    let potential_name = split.next().unwrap().trim();
    let name;
    let module_type;
    if potential_name.starts_with('%') {
        name = trim_name(potential_name);
        module_type = ModuleType::FlipFlop(FlipFlopState::Off);
    } else if potential_name.starts_with('&') {
        name = trim_name(potential_name);
        let sources = find_sources(&name, input);
        let mut history = HashMap::new();
        for source in sources {
            history.insert(source, Pulse::Low);
        }
        module_type = ModuleType::Conjunction { history };
    } else if potential_name == "broadcaster" {
        name = potential_name.to_string();
        module_type = ModuleType::Broadcast;
    } else {
        name = potential_name.to_string();
        module_type = ModuleType::TestType;
    }

    let connections = split
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.trim().to_string())
        .collect();

    Module {
        name: name.to_string(),
        module_type,
        connections,
        output: None,
    }
}

fn parse_data(input: &str) -> Vec<Module> {
    input.lines().map(|x| parse_line(x, input)).collect()
}

/// # Panics
#[must_use]
pub fn part1(input: &str) -> String {
    let modules = parse_data(input);
    let (cycle_length, low, hi) = process("broadcaster", &Pulse::Low, modules);
    let result = (1000 / cycle_length) * (1000 / cycle_length) * hi * low;
    format!("{result}")
}

fn process_part2(
    name: &str,
    pulse: &Pulse,
    mut modules: Vec<Module>,
    searched_cycle: &str,
) -> usize {
    // For test_case 2 in part 1
    modules.push(Module {
        name: "output".to_string(),
        module_type: ModuleType::TestType,
        connections: Vec::new(),
        output: None,
    });
    // For real input
    modules.push(Module {
        name: "rx".to_string(),
        module_type: ModuleType::TestType,
        connections: Vec::new(),
        output: None,
    });

    let mut button_pushes = 0;

    'outer: loop {
        let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();
        queue.push_back((name.to_string(), pulse.clone(), "button".to_string()));

        while !queue.is_empty() {
            let value = modules.iter().find(|x| x.name == searched_cycle).unwrap();
            if value.output == Some(Pulse::High) {
                break 'outer;
            }
            let (state, input, source) = queue.pop_front().unwrap();
            let mut current_state = modules
                .iter_mut()
                .find(|x| x.name == state)
                .unwrap()
                .clone();

            if std::mem::discriminant(&current_state.module_type)
                == std::mem::discriminant(&ModuleType::FlipFlop(FlipFlopState::On))
                && input == Pulse::High
            {
                continue;
            }

            current_state.handle_input(&input, source);
            for connection in current_state.connections.clone() {
                queue.push_back((
                    connection.to_string(),
                    current_state.output.as_ref().unwrap().clone(),
                    current_state.name.clone(),
                ));
            }
            modules = modules
                .clone()
                .into_iter()
                .map(|x| {
                    if x.name == current_state.name {
                        current_state.clone()
                    } else {
                        x
                    }
                })
                .collect();
        }

        button_pushes += 1;
    }
    println!("{searched_cycle} -> {button_pushes}");
    button_pushes + 1
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    let mut bigger = x;
    let mut smaller = y;
    if smaller > bigger {
        std::mem::swap(&mut x, &mut y);
    }

    loop {
        let res = bigger % smaller;
        if res == 0 {
            return smaller;
        }

        bigger = smaller;
        smaller = res;
    }
}

/// # Panics
#[must_use]
pub fn part2(input: &str) -> String {
    let modules = parse_data(input);
    // By watching my input I can see that rx is conjunction type module with 4 inputs: xn, qn, xf
    // and zl. So I need to check cycles when they turn to be On at the same time.

    let cycle1 = process_part2("broadcaster", &Pulse::Low, modules.clone(), "xn");
    let cycle2 = process_part2("broadcaster", &Pulse::Low, modules.clone(), "qn");
    let cycle3 = process_part2("broadcaster", &Pulse::Low, modules.clone(), "xf");
    let cycle4 = process_part2("broadcaster", &Pulse::Low, modules.clone(), "zl");

    let result = lcm(lcm(cycle1, cycle2), lcm(cycle3, cycle4));
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a";

    const TEST_DATA2: &str = "broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output";

    #[test]
    fn test_case_part1_1() {
        assert_eq!("32000000", part1(TEST_DATA));
    }

    #[test]
    fn test_case_part1_2() {
        assert_eq!("11687500", part1(TEST_DATA2));
    }
}
