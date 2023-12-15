use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Operation {
    Value(isize),
    Addition(String, String),
    Multiplication(String, String),
    Subtraction(String, String),
    Division(String, String),
}

#[derive(Debug, PartialEq)]
struct Equation {
    lhs: String,
    rhs: Operation,
}

impl Equation {
    fn new(lhs: String, rhs: Operation) -> Self {
        Equation { lhs, rhs }
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let mut split = value.split(':');
        let lhs = split.next().unwrap().trim().to_owned();
        let rhs = split.next().unwrap().trim();

        let mut rhs_split = rhs.split(' ');
        let first_part = rhs_split.next().unwrap().to_owned();
        if first_part.chars().next().unwrap().is_ascii_digit() {
            return Equation {
                lhs,
                rhs: Operation::Value(rhs.parse().unwrap()),
            };
        }

        let operand = rhs_split.next().unwrap();
        let second_part = rhs_split.next().unwrap().to_owned();
        let rhs = match operand {
            "+" => Operation::Addition(first_part, second_part),
            "-" => Operation::Subtraction(first_part, second_part),
            "*" => Operation::Multiplication(first_part, second_part),
            "/" => Operation::Division(first_part, second_part),
            _ => {
                unreachable!("Equation from String: Impossible state")
            }
        };
        Equation { lhs, rhs }
    }
}

fn eval_equation<'a>(
    unknown: &'a str,
    equations: &'a [Equation],
    cache: &mut HashMap<&'a str, isize>,
) -> isize {
    if cache.contains_key(unknown) {
        return *cache.get(unknown).unwrap();
    }
    let equation: &Equation = equations.iter().find(|e| e.lhs == unknown).unwrap();
    let result = match &equation.rhs {
        Operation::Value(x) => *x,
        Operation::Multiplication(x, y) => {
            eval_equation(x, equations, cache) * eval_equation(y, equations, cache)
        }
        Operation::Addition(x, y) => {
            eval_equation(x, equations, cache) + eval_equation(y, equations, cache)
        }
        Operation::Division(x, y) => {
            eval_equation(x, equations, cache) / eval_equation(y, equations, cache)
        }
        Operation::Subtraction(x, y) => {
            eval_equation(x, equations, cache) - eval_equation(y, equations, cache)
        }
    };
    cache.insert(unknown, result);
    result
}

pub fn part1(input: &str) -> isize {
    let equations: Vec<Equation> = input.lines().map(Equation::from).collect();
    let mut cache: HashMap<&str, isize> = HashMap::new();
    eval_equation("root", &equations, &mut cache)
}

#[must_use] pub fn part2(_input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32";

    #[test]
    fn parse_value() {
        let row = "humn: 5";
        assert_eq!(
            Equation::new("humn".to_owned(), Operation::Value(5)),
            Equation::from(row)
        );
    }

    #[test]
    fn parse_operation() {
        let row = "sjmn: drzm * dbpl";
        assert_eq!(
            Equation::new(
                "sjmn".to_owned(),
                Operation::Multiplication("drzm".to_owned(), "dbpl".to_owned())
            ),
            Equation::from(row)
        );
    }

    #[test]
    fn eval_value() {
        let equations = vec![Equation::new(String::from("x"), Operation::Value(5))];
        let mut cache = HashMap::new();
        assert_eq!(5, eval_equation("x", &equations, &mut cache));
    }

    #[test]
    fn eval_operation() {
        let equations = vec![
            Equation::new(
                String::from("x"),
                Operation::Multiplication(String::from("y"), String::from("z")),
            ),
            Equation::new(String::from("y"), Operation::Value(2)),
            Equation::new(String::from("z"), Operation::Value(3)),
        ];
        let mut cache = HashMap::new();
        assert_eq!(6, eval_equation("x", &equations, &mut cache));
    }

    #[test]
    fn test_case_part1() {
        let equations: Vec<Equation> = TEST_DATA.lines().map(Equation::from).collect();
        let mut cache = HashMap::new();
        assert_eq!(152, eval_equation("root", &equations, &mut cache))
    }
}
