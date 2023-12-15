#[allow(clippy::cast_possible_wrap)]
fn indices_with_negatives(x: isize, m: usize) -> usize {
    let result = x % (m as isize);
    if result >= 0 {
        result.try_into().unwrap()
    } else {
        (m as isize + result).try_into().unwrap()
    }
}

#[allow(clippy::cast_possible_wrap)]
fn move_index(indices: &mut Vec<usize>, arr: &mut Vec<isize>, length: usize) {
    let index = indices.pop().unwrap();
    let value = arr[index];
    let new_index: usize = indices_with_negatives(index as isize + value, length - 1);
    arr.remove(index);
    arr.insert(new_index, value);
    let new_indices: Vec<usize> = indices
        .iter()
        .map(|x| if x <= &new_index { *x - 1 } else { *x })
        .collect();
    indices.clear();
    indices.extend(new_indices);
}

fn mix_file(indices: &mut Vec<usize>, arr: &mut Vec<isize>, length: usize) {
    while !indices.is_empty() {
        move_index(indices, arr, length);
    }
}

fn grove_coordinates(arr: &[isize], length: usize) -> isize {
    let zero_index = arr.iter().position(|&x| x == 0).unwrap();
    let x = arr[(zero_index + 1000) % length];
    let y = arr[(zero_index + 2000) % length];
    let z = arr[(zero_index + 3000) % length];
    x + y + z
}

#[must_use] pub fn part1(input: &str) -> isize {
    let mut arr: Vec<isize> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    let length = arr.len();
    let mut indices: Vec<usize> = (0..length).rev().collect();
    mix_file(&mut indices, &mut arr, length);
    grove_coordinates(&arr, length)
}

#[must_use] pub fn part2(_input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_DATA: &str = "1
    2
    -3
    3
    -2
    0
    4";

    #[test]
    fn move_index_test() {
        let mut inidices = vec![6, 5, 4, 3, 2, 1, 0];
        let mut arr = vec![1, 2, -3, 3, -2, 0, 4];
        move_index(&mut inidices, &mut arr, 7);
        assert_eq!(vec![2, 1, -3, 3, -2, 0, 4], arr);
    }

    #[test]
    fn move_index_test2() {
        let mut inidices = vec![6, 5, 4, 3, 2, 1, 0];
        let mut arr = vec![1, 2, -3, 3, -2, 0, 4];
        move_index(&mut inidices, &mut arr, 7);
        move_index(&mut inidices, &mut arr, 7);
        move_index(&mut inidices, &mut arr, 7);
        assert_eq!(vec![1, 2, 3, -2, -3, 0, 4], arr);
    }

    #[test]
    fn mod_with_negatives_test() {
        assert_eq!(2, indices_with_negatives(9, 7));
        assert_eq!(5, indices_with_negatives(-9, 7));
        assert_eq!(2, indices_with_negatives(-145, 7));
    }

    #[test]
    fn test_case_part1() {
        assert_eq!(3, part1(TEST_DATA));
    }
}
