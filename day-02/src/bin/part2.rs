use itertools::Itertools;
fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
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
            if !check(&report) {
                for i in 0..report.len() {
                    let mut retry = report.clone();
                    retry.remove(i);
                    if check(&retry) {
                        return true;
                    }
                }
                return false;
            }
            return true;
        })
        .fold(0, |acc, passed| if passed { acc + 1 } else { acc })
        .to_string()
}

fn check(report: &Vec<i32>) -> bool {
    let increasing = report[0] < report[1];
    for (a, b) in report.iter().tuple_windows() {
        if (increasing && a > b) || (!increasing && a < b) {
            return false;
        }
        if a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3 {
            continue;
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, "4");
    }
}
