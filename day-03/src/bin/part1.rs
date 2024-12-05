use regex::Regex;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)").unwrap();
    let mut results = vec![];
    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
    }
    results.iter().map(|(x, y)| x * y).sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(result, "161");
    }
}
