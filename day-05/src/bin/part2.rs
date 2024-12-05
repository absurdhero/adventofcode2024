use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let printer_rules = PrinterRules::from_string(input);
    let valid = printer_rules.valid();

    valid.iter().for_each(|line| {
        let mid = line[(line.len()) / 2];
        println!("{}: {}", line.iter().join(","), mid);
    });
    valid.iter().map(|line| line[(line.len())/2]).sum::<i32>().to_string()
}

#[derive(Debug)]
struct PrinterRules {
    rules: Vec<(i32, i32)>,
    lines: Vec<Vec<i32>>,
}

impl PrinterRules {
    fn from_string(input: &str) -> PrinterRules {
        let (rules_str, pages_str) = input.split_once("\n\n").unwrap();
        let rules_re = Regex::new("(\\d+)\\|(\\d+)\n").unwrap();
        let rules: Vec<(i32, i32)> = rules_re
            .captures_iter(rules_str)
            .map(|c| c.extract::<2>())
            .map(|(_a, b)| (b[0].parse().unwrap(), b[1].parse().unwrap()))
            .collect();
        let lines: Vec<Vec<i32>> = pages_str
            .lines()
            .map(|l| l.split(",").map(|s| s.parse().unwrap()).collect())
            .collect();

        PrinterRules { rules, lines }
    }

    fn valid(&self) -> Vec<Vec<i32>> {
        self.lines
            .iter()
            .filter(|l| !self.check_rules(l))
            .map(|l| {
                let mut fixed = l.clone();
                for i in 0..l.len() {
                    fixed = self.fix(&fixed);
                }
                fixed
            } )
            .collect()
    }

    fn check_rules(&self, line: &Vec<i32>) -> bool {
        for rule in &self.rules {
            if let Some((first_pos, _val)) = line.iter().find_position(|x| **x == rule.0) {
                if let Some((second_pos, _val)) = line.iter().find_position(|x| **x == rule.1) {
                    if first_pos > second_pos {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn fix(&self, line: &Vec<i32>) -> Vec<i32> {
        let mut line = line.clone();
        for rule in &self.rules {
            if let Some((first_pos, &first_val)) = line.iter().find_position(|x| **x == rule.0) {
                if let Some((second_pos, &second_val)) = line.iter().find_position(|x| **x == rule.1) {
                    if first_pos > second_pos {
                        line.swap(first_pos, second_pos);
                        // line.remove(first_pos);
                        // line.insert(second_pos, first_val);
                    }
                }
            }
        }
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, "123");
    }
}
