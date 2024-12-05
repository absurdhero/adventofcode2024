use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let list1: Vec<&str> = input.split_whitespace().step_by(2).collect();
    let mut list2: Vec<&str> = input.split_whitespace().skip(1).step_by(2).collect();

    let mut left_list: Vec<i32> = vec![];
    let mut frequencies: HashMap<i32, i32> = HashMap::new();

    for (&a, &b) in list1.iter().zip(list2.iter()) {
        let a = a.parse::<i32>().unwrap();
        left_list.push(a);
        let b = b.parse::<i32>().unwrap();
        *frequencies.entry(b).or_insert(0) += 1;
    }

    let mut sum = 0;
    for x in left_list {
        if let Some(num) = frequencies.get(&x) {
            sum += x * num;
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, "31");
    }
}
