fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut list1: Vec<&str> = input.split_whitespace().step_by(2).collect();
    list1.sort();
    let mut list2: Vec<&str> = input.split_whitespace().skip(1).step_by(2).collect();
    list2.sort();
    // dbg!(&list1);
    // dbg!(&list2);

    let mut sum: u64 = 0;
    for (&a, &b) in list1.iter().zip(list2.iter()) {
        sum += b.parse::<i64>().unwrap().abs_diff(a.parse::<i64>().unwrap());
    }
    dbg!(sum);
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, "11");
    }
}

