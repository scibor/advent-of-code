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
            (_, ModuleType::Broadcast) => self.output = Some(pulse.clone()),
            (_, ModuleType::TestType) => self.output = Some(pulse.clone()),
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
                if history.values().all(|x| *x == Pulse::High) {
                    self.output = Some(Pulse::Low)
                } else {
                    self.output = Some(Pulse::High)
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

fn process(name: String, pulse: Pulse, mut modules: Vec<Module>) -> Vec<Module> {
    let mut low = 0;
    let mut high = 0;

    modules.push(Module {
        name: "output".to_string(),
        module_type: ModuleType::TestType,
        connections: Vec::new(),
        output: None,
    });

    let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();
    queue.push_back((name, pulse.clone(), "button".to_string()));

    while !queue.is_empty() {
        let (state, input, source) = queue.pop_front().unwrap();
        println!("{:?}", &modules);
        println!("{source} {:?} {state}", input);
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
            ))
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

    println!("low: {low}");
    println!("high: {high}");

    Vec::new()
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
    if potential_name.starts_with("%") {
        name = trim_name(potential_name);
        module_type = ModuleType::FlipFlop(FlipFlopState::Off);
    } else if potential_name.starts_with("&") {
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

pub fn part1(input: &str) -> String {
    let mut modules = parse_data(input);
    process("broadcaster".to_string(), Pulse::Low, modules);
    let result = 0;
    format!("{result}")
}

pub fn part2(_input: &str) -> String {
    let result = 0;
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
    #[ignore = "not yet"]
    fn test_case_part1_1() {
        assert_eq!("32000000", part1(TEST_DATA));
    }

    #[test]
    fn test_case_part1_2() {
        assert_eq!("11687500", part1(TEST_DATA2));
    }

    #[test]
    #[ignore = "not yet"]
    fn test_case_part2() {
        assert_eq!("1", part1(TEST_DATA));
    }
}
