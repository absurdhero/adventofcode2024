fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let equations: Vec<Equation> = input.lines().map(|line| {
        let parts = line.split_once(':').unwrap();
        Equation::new(
            parts.0.parse().unwrap(),
            parts.1.trim().split(' ').map(|p| p.parse().unwrap()).collect(),
        )
    }).collect();

    equations
        .iter()
        .filter(|e| e.valid_operators())
        .fold(0i64, |a, b| a + b.total)
        .to_string()
}

struct Equation {
    total: i64,
    elements: Vec<i64>,
}



impl Equation {
    fn new(total: i64, elements: Vec<i64>) -> Self {
        Equation { total, elements }
    }

    pub fn valid_operators(&self) -> bool {
        fn generate(elements: &[i64], total: i64) -> Vec<i64> {
            if elements.len() == 0 {
                vec![total]
            } else {
                let mut combos = vec![];
                combos.append(&mut generate(&elements[1..], total + elements[0]));
                combos.append(&mut generate(&elements[1..], total * elements[0]));
                combos
            }
        }

        generate(&self.elements, 0).contains(&self.total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(result, "3749");
    }
}
