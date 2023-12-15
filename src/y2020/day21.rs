use std::{
    collections::{HashMap, HashSet},
    ops::BitAnd,
};

fn get_allergens(foods: &[Food]) -> HashSet<String> {
    foods.iter().flat_map(|f| f.allergens.clone()).collect()
}

fn create_alergen_map(
    foods: &[Food],
    allergens: &HashSet<String>,
) -> HashMap<String, HashSet<String>> {
    let mut result = HashMap::new();
    for allergen in allergens {
        let mut possible_foods = HashSet::new();
        for food in foods {
            if food.allergens.contains(allergen) {
                if possible_foods.is_empty() {
                    possible_foods = food.ingredients.clone();
                } else {
                    possible_foods = possible_foods.bitand(&food.ingredients);
                }
            }
        }
        result.insert(allergen.clone(), possible_foods.into_iter().collect());
    }
    result
}

fn reduce_allergen_map(
    allergen_map: &mut HashMap<String, HashSet<String>>,
) -> HashMap<String, HashSet<String>> {
    let mut fixed: Vec<String> = Vec::new();

    while allergen_map
        .iter()
        .any(|(k, v)| v.len() == 1 && !fixed.contains(k))
    {
        let (k, v) = allergen_map
            .iter()
            .find(|(k, v)| v.len() == 1 && !fixed.contains(k))
            .unwrap();

        let mut modifications = Vec::new();

        for (key, value) in &*allergen_map {
            if key != k && value.contains(v.iter().next().unwrap()) {
                modifications.push((key.clone(), v.clone()));
            }
        }

        fixed.push(k.clone());

        for (key, modification) in modifications {
            allergen_map
                .get_mut(&key)
                .unwrap()
                .retain(|elem| elem != modification.iter().next().unwrap());
        }
    }

    allergen_map.clone()
}

/// # Panics
#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn create_answer_for_part_2(reduced_map: &HashMap<String, HashSet<String>>) -> String {
    let mut pairs: Vec<(String, String)> = reduced_map
        .iter()
        .map(|(k, v)| (k.clone(), v.iter().next().unwrap().clone()))
        .collect();
    pairs.sort_by(|(k, _), (k2, _)| k.cmp(k2));
    let string_vec: Vec<String> = pairs.into_iter().map(|(_k, v)| v).collect();
    string_vec.join(",")
}

/// # Panics
pub fn part1_and_2(input: &str) -> (usize, String) {
    let foods: Vec<Food> = input.lines().map(parse_row).collect();
    let allergens = get_allergens(&foods);
    let mut allergen_map = create_alergen_map(&foods, &allergens);

    let reduced_map = reduce_allergen_map(&mut allergen_map);
    let fixed_allergens: Vec<String> = reduced_map
        .values()
        .map(|v| v.iter().next().unwrap().clone())
        .collect();

    let mut result = 0;

    for food in foods {
        result += food
            .ingredients
            .iter()
            .filter(|i| !fixed_allergens.contains(i))
            .count();
    }
    let part2 = create_answer_for_part_2(&reduced_map);

    (result, part2)
}

#[derive(Debug, PartialEq)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn parse_row(row: &str) -> Food {
    let mut split = row.split(" (contains ");
    let ingredients = split
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_owned())
        .collect();
    let allergens = split
        .next()
        .unwrap()
        .replace(')', "")
        .split(", ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_owned())
        .collect();
    Food {
        ingredients,
        allergens,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_CASE_INPUT: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn parse_row_test() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)";
        let expected = Food {
            ingredients: HashSet::from_iter(
                vec![
                    "mxmxvkd".to_owned(),
                    "kfcds".to_owned(),
                    "sqjhc".to_owned(),
                    "nhms".to_owned(),
                ]
                .into_iter(),
            ),
            allergens: HashSet::from_iter(vec!["dairy".to_owned(), "fish".to_owned()].into_iter()),
        };
        assert_eq!(expected, parse_row(input));
    }

    #[test]
    fn get_allergens_test() {
        let foods: Vec<Food> = TEST_CASE_INPUT.lines().map(parse_row).collect();
        let allergens = get_allergens(&foods);
        let mut expected = HashSet::new();
        expected.insert(String::from("dairy"));
        expected.insert(String::from("soy"));
        expected.insert(String::from("fish"));
        assert_eq!(expected, allergens);
    }

    #[test]
    fn create_alergen_map_test() {
        let foods: Vec<Food> = TEST_CASE_INPUT.lines().map(parse_row).collect();
        let allergens = get_allergens(&foods);
        let allergen_map = create_alergen_map(&foods, &allergens);
        let mut expected = HashMap::new();
        expected.insert(
            "fish".to_owned(),
            HashSet::from_iter(vec![String::from("mxmxvkd"), String::from("sqjhc")].into_iter()),
        );
        expected.insert(
            "dairy".to_owned(),
            HashSet::from_iter(vec![String::from("mxmxvkd")]),
        );
        expected.insert(
            "soy".to_owned(),
            HashSet::from_iter(vec![String::from("sqjhc"), String::from("fvjkl")].into_iter()),
        );
        assert_eq!(expected, allergen_map);
    }

    #[test]
    fn reduce_allergen_map_test() {
        let foods: Vec<Food> = TEST_CASE_INPUT.lines().map(parse_row).collect();
        let allergens = get_allergens(&foods);
        let mut allergen_map = create_alergen_map(&foods, &allergens);
        let reduced_map = reduce_allergen_map(&mut allergen_map);
        let mut expected = HashMap::new();
        expected.insert(
            "fish".to_owned(),
            HashSet::from_iter(vec![String::from("sqjhc")].into_iter()),
        );
        expected.insert(
            "dairy".to_owned(),
            HashSet::from_iter(vec![String::from("mxmxvkd")]),
        );
        expected.insert(
            "soy".to_owned(),
            HashSet::from_iter(vec![String::from("fvjkl")].into_iter()),
        );
        assert_eq!(expected, reduced_map);
    }

    #[test]
    fn test_case_part1() {
        assert_eq!(5, part1_and_2(TEST_CASE_INPUT).0);
    }

    #[test]
    fn part2_answer_test() {
        assert_eq!(
            "mxmxvkd,sqjhc,fvjkl".to_owned(),
            part1_and_2(TEST_CASE_INPUT).1
        );
    }
}
