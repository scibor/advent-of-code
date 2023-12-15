use std::collections::HashMap;

fn hash(input: &str) -> usize {
    let mut result = 0;
    for c in input.chars() {
        result = ((result + c as usize) * 17) % 256;
    }
    result
}

#[must_use] pub fn part1(input: &str) -> String {
    let fragments: Vec<&str> = input.trim().split(',').collect();
    let result: usize = fragments.iter().map(|x| hash(x.trim())).sum();
    format!("{result}")
}

#[derive(Debug, PartialEq)]
struct Lens {
    label: String,
    label_hash: usize,
    mode: char,
    focal_length: Option<usize>,
}

impl Lens {
    fn parse(input: &str) -> Lens {
        if input.contains('=') {
            let mut split = input.split('=');
            let label = split.next().unwrap().to_owned();
            let label_hash = hash(&label);
            let focal_length = Some(split.next().unwrap().parse::<usize>().unwrap());
            return Lens {
                label,
                label_hash,
                mode: '=',
                focal_length,
            };
        } else if input.contains('-') {
            let mut split = input.split('-');
            let label = split.next().unwrap().to_owned();
            let label_hash = hash(&label);
            let focal_length = None;
            return Lens {
                label,
                label_hash,
                mode: '-',
                focal_length,
            };
        }
        unreachable!();
    }
}

fn focusing_power(boxes: HashMap<usize, Vec<Lens>>) -> usize {
    let mut result = 0;
    for (b, v) in boxes {
        for (i, e) in v.iter().enumerate() {
            result += (b + 1) * (i + 1) * e.focal_length.unwrap();
        }
    }
    result
}

#[must_use] pub fn part2(input: &str) -> String {
    let split = input.trim().split(',');
    let mut lenses: Vec<Lens> = Vec::new();
    for s in split {
        lenses.push(Lens::parse(s));
    }

    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();

    for i in 0..256 {
        boxes.insert(i, Vec::new());
    }

    for lens in lenses {
        if lens.mode == '-'
            && boxes
                .get(&lens.label_hash)
                .unwrap()
                .iter()
                .any(|l: &Lens| l.label == lens.label)
        {
            boxes
                .get_mut(&lens.label_hash)
                .unwrap()
                .retain(|l| l.label != lens.label);
        } else if lens.mode == '=' {
            if boxes
                .get(&lens.label_hash)
                .unwrap()
                .iter()
                .any(|l: &Lens| l.label == lens.label)
            {
                let mut_iter = boxes.get_mut(&lens.label_hash).unwrap().iter_mut();

                for l in mut_iter {
                    if l.label == lens.label {
                        l.focal_length = lens.focal_length;
                    }
                }
            } else {
                boxes.get_mut(&lens.label_hash).unwrap().push(lens);
            }
        }
    }

    let result = focusing_power(boxes);
    format!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_case_part1() {
        assert_eq!("1320", part1(TEST_DATA));
    }

    #[test]
    fn parse_lense_test() {
        let input1 = "rn=1";
        let input2 = "cm-";
        let expected1 = Lens {
            label: String::from("rn"),
            label_hash: 0,
            mode: '=',
            focal_length: Some(1),
        };
        let expected2 = Lens {
            label: String::from("cm"),
            label_hash: 0,
            mode: '-',
            focal_length: None,
        };
        assert_eq!(expected1, Lens::parse(input1));
        assert_eq!(expected2, Lens::parse(input2));
    }

    #[test]
    fn test_case_part2() {
        assert_eq!("145", part2(TEST_DATA));
    }
}
