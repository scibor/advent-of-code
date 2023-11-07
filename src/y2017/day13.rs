#[derive(Debug, PartialEq, Clone)]
enum ScanningDirection {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Clone)]
struct Layer {
    depth: usize,
    range: usize,
    scanner_pos: usize,
    scanning_direction: ScanningDirection,
}

impl Layer {
    fn new(depth: usize, range: usize) -> Self {
        let scanner_pos = if range == 0 { std::usize::MAX } else { 0 };
        Layer {
            depth,
            range,
            scanner_pos,
            scanning_direction: ScanningDirection::Up,
        }
    }

    fn move_scanner(&mut self) {
        if self.range == 0 {
            return;
        }

        if self.scanner_pos == self.range - 1 && self.scanning_direction == ScanningDirection::Up {
            self.scanning_direction = ScanningDirection::Down;
            return self.move_scanner();
        }

        if self.scanner_pos == 0 && self.scanning_direction == ScanningDirection::Down {
            self.scanning_direction = ScanningDirection::Up;
            return self.move_scanner();
        }

        if self.scanning_direction == ScanningDirection::Up {
            self.scanner_pos += 1;
        }

        if self.scanning_direction == ScanningDirection::Down {
            self.scanner_pos -= 1;
        }
    }
}

fn parse_row(row: &str) -> Layer {
    let split: Vec<&str> = row.split(':').collect();
    Layer::new(
        split.first().unwrap().trim().parse::<usize>().unwrap(),
        split.get(1).unwrap().trim().parse::<usize>().unwrap(),
    )
}

fn parse_layers(input: &str) -> Vec<Layer> {
    input.lines().map(parse_row).collect()
}

fn add_empty_layers(layers: &[Layer]) -> Vec<Layer> {
    let max = &layers.iter().map(|x| x.depth).max().unwrap();
    let mut result: Vec<Layer> = Vec::new();
    let mut index = 0;
    for i in 0..=*max {
        if layers.get(index).unwrap().depth == i {
            result.push(layers.get(index).unwrap().clone());
            index += 1;
        } else {
            result.push(Layer::new(i, 0));
        }
    }
    result
}

fn move_scanners(layers: &mut [Layer]) {
    for layer in &mut *layers {
        layer.move_scanner();
    }
}

fn trip_severity(mut layers: Vec<Layer>) -> usize {
    let mut result = 0;
    let length = layers.len();

    for i in 0..length {
        if layers[i].scanner_pos == 0 {
            result += layers[i].depth * layers[i].range;
        }
        move_scanners(&mut layers);
    }
    result
}

pub fn part1(input: &str) -> usize {
    let layers: Vec<Layer> = add_empty_layers(&parse_layers(input));
    trip_severity(layers)
}

#[derive(Debug, PartialEq)]
struct Congruence {
    modulo: usize,
    not_equal: usize,
}

impl Congruence {
    fn new(modulo: usize, not_equal: usize) -> Self {
        Congruence { modulo, not_equal }
    }

    fn to_predicate(&self, x: usize) -> bool {
        x % self.modulo != self.not_equal
    }
}

fn get_congruences(layers: Vec<Layer>) -> Vec<Congruence> {
    layers
        .into_iter()
        .filter(|l| l.range != 0)
        .map(|l| {
            let modulo = (l.range - 1) * 2;
            let not_equal = if modulo > l.depth {
                (modulo - l.depth) % modulo
            } else {
                let diff = l.depth % modulo;
                (modulo - diff) % modulo
            };
            Congruence::new(modulo, not_equal)
        })
        .collect()
}

fn find_number(congruences: &[Congruence]) -> usize {
    let result = (0..usize::MAX)
        .find(|x| congruences.iter().all(|c| c.to_predicate(*x)))
        .unwrap();
    result
}

fn delay_start(layers: Vec<Layer>) -> usize {
    let congruences: Vec<Congruence> = get_congruences(layers);
    find_number(&congruences)
}

fn trip_severity_with_delay(mut layers: Vec<Layer>, delay: usize) -> usize {
    let mut result = 0;
    let length = layers.len();

    for i in 0..delay + length {
        if i >= delay && layers[i - delay].scanner_pos == 0 {
            result += layers[i - delay].depth * layers[i - delay].range;
        }
        move_scanners(&mut layers);
    }
    result
}

pub fn part2(input: &str) -> usize {
    let layers: Vec<Layer> = add_empty_layers(&parse_layers(input));
    delay_start(layers.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_CASE_INPUT: &str = r"0: 3
    1: 2
    4: 4
    6: 4";

    #[test]
    fn parse_row_test() {
        let input = "44: 14";
        assert_eq!(Layer::new(44, 14), parse_row(input));
    }

    #[test]
    fn parse_layers_test() {
        let expected = vec![
            Layer::new(0, 3),
            Layer::new(1, 2),
            Layer::new(4, 4),
            Layer::new(6, 4),
        ];
        assert_eq!(expected, parse_layers(TEST_CASE_INPUT));
    }

    #[test]
    fn add_empty_layers_test() {
        let expected = vec![
            Layer::new(0, 3),
            Layer::new(1, 2),
            Layer::new(2, 0),
            Layer::new(3, 0),
            Layer::new(4, 4),
            Layer::new(5, 0),
            Layer::new(6, 4),
        ];
        assert_eq!(expected, add_empty_layers(&parse_layers(TEST_CASE_INPUT)));
    }

    #[test]
    fn move_scanner_empty_layer() {
        let mut layer = Layer::new(3, 0);
        layer.move_scanner();
        assert_eq!(std::usize::MAX, layer.scanner_pos);
    }

    #[test]
    fn move_scanner_regular_layer() {
        let mut layer = Layer::new(3, 3);
        layer.move_scanner();
        assert_eq!(1, layer.scanner_pos);
    }

    #[test]
    fn move_scanner_wrap() {
        let mut layer = Layer::new(3, 3);
        assert_eq!(0, layer.scanner_pos);
        layer.move_scanner();
        assert_eq!(1, layer.scanner_pos);
        layer.move_scanner();
        assert_eq!(2, layer.scanner_pos);
        layer.move_scanner();
        assert_eq!(1, layer.scanner_pos);
    }

    #[test]
    fn test_case_part1() {
        let layers = add_empty_layers(&parse_layers(TEST_CASE_INPUT));
        assert_eq!(24, trip_severity(layers));
    }

    #[test]
    fn test_congruence() {
        let congruence = Congruence::new(6, 2);
        assert!(congruence.to_predicate(10));
        assert!(!congruence.to_predicate(8));
    }

    #[test]
    fn test_get_congruences() {
        let layers = add_empty_layers(&parse_layers(TEST_CASE_INPUT));
        let expected = vec![
            Congruence::new(4, 0),
            Congruence::new(2, 1),
            Congruence::new(6, 2),
            Congruence::new(6, 0),
        ];
        assert_eq!(expected, get_congruences(layers));
    }

    #[test]
    fn find_number_fullfilling_congruences() {
        let congruences = vec![
            Congruence::new(4, 0),
            Congruence::new(2, 1),
            Congruence::new(6, 2),
            Congruence::new(6, 0),
        ];
        assert_eq!(10, find_number(&congruences));
    }

    #[test]
    fn test_case_part2() {
        let layers = add_empty_layers(&parse_layers(TEST_CASE_INPUT));
        assert_eq!(10, delay_start(layers));
    }
}
