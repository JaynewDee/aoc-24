fn main() {
    let input = read_input();
    let mut parsed = parse_input(input);

    for equation in parsed.iter_mut() {
        equation.test();
    }

    let mut total = 0;
    for equation in parsed {
        if equation.is_possible {
            total += equation.test_value;
        }
    }

    println!("{}", total);
}

fn read_input() -> String {
    std::fs::read_to_string("../input/day7.txt").expect("Failed to read input at provided path.")
}

#[derive(Debug)]
struct Equation {
    test_value: u64,
    nums: Vec<u64>,
    is_possible: bool,
}

impl Equation {
    pub fn test(&mut self) {
        if Self::try_combination(&self.nums, self.test_value) {
            self.is_possible = true;
        }
    }

    fn try_combination(numbers: &[u64], target: u64) -> bool {
        // Recursive helper
        fn recurse(
            numbers: &[u64],
            target: u64,
            current_value: u64,
            operators: &mut Vec<char>,
            results: &mut Vec<char>,
        ) -> bool {
            if numbers.is_empty() {
                if current_value == target {
                    *results = operators.clone();
                    return true;
                }
                return false;
            }

            operators.push('+');
            if recurse(
                &numbers[1..],
                target,
                current_value + numbers[0],
                operators,
                results,
            ) {
                return true;
            }
            operators.pop();

            operators.push('*');
            if recurse(
                &numbers[1..],
                target,
                current_value * numbers[0],
                operators,
                results,
            ) {
                return true;
            }
            operators.pop();

            false
        }

        let mut operators = Vec::new();
        let mut results = Vec::new();
        if recurse(
            &numbers[1..],
            target,
            numbers[0],
            &mut operators,
            &mut results,
        ) {
            true
        } else {
            false
        }
    }
}

fn parse_input(input: String) -> Vec<Equation> {
    let mut equations: Vec<Equation> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(":").map(String::from);
        let part1 = parts.next().unwrap();
        let test_value = part1.parse::<u64>().unwrap();
        let part2 = parts.next().unwrap();
        let part2_pieces = part2.split_whitespace();
        let mut nums: Vec<u64> = Vec::new();

        for num in part2_pieces {
            let n = num.parse::<u64>().unwrap();
            nums.push(n);
        }
        equations.push(Equation {
            test_value,
            nums,
            is_possible: false,
        });
    }

    equations
}
