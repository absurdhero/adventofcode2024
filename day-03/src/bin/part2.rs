use regex::Regex;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let re =
        Regex::new(r"(do)\(\)()()|(don't)\(\)()()|(mul)\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)")
            .unwrap();
    let mut results = vec![];
    for (_, [op, x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((
            op,
            x.parse::<i32>().unwrap_or(0),
            y.parse::<i32>().unwrap_or(0),
        ));
    }
    results
        .iter()
        .fold((true, 0), |(enabled, sum), item| {
            if item.0 == "do" {
                (true, sum)
            } else if item.0 == "don't" {
                (false, sum)
            } else if enabled  {
                (true, sum + item.1 * item.2)
            } else {
                (false, sum)
            }
        }).1.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result =
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, "48");
    }
}
