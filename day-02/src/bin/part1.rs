use itertools::Itertools;
fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input
        .split("\n")
        .map(|report| {
            report
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|report| {
            if report.len() < 2 {
                return true;
            }
            let increasing = report[0] < report[1];
            for (a, b) in report.into_iter().tuple_windows() {
                if (increasing && a > b) || (!increasing && a < b) {
                    return false;
                }
                if a.abs_diff(b) >= 1 && a.abs_diff(b) <= 3 {
                    continue
                } else {
                    return false
                }
            }
            return true
        })
        .fold(0, |acc, x| if x { acc + 1 } else { acc })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, "2");
    }
}
