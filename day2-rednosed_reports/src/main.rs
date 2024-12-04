fn main() {
    part1::execute();
    part2::execute();
}

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).expect("Failed to read input at provided path.")
}

#[derive(Debug, Clone)]
struct Report {
    nums: Vec<i8>,
    is_safe: bool,
}

impl Report {
    pub fn new(nums: Vec<i8>) -> Self {
        Self {
            nums,
            is_safe: false,
        }
    }

    pub fn check_safety(&mut self) -> bool {
        let mut prev = &self.nums[0];
        let mut is_decreasing = false;
        let mut is_increasing = false;

        for num in &self.nums[1..] {
            if prev.abs_diff(*num) > 3 || prev == num {
                return false;
            }

            if prev > num {
                is_decreasing = true;
                if is_increasing {
                    return false;
                }
            }

            if prev < num {
                is_increasing = true;
                if is_decreasing {
                    return false;
                }
            }

            prev = num;
        }

        self.is_safe = true;
        return self.is_safe;
    }

    pub fn use_dampener(&mut self, mut current: usize) {
        // recursive terminating case
        if current == self.nums.len() {
            return;
        }

        let mut dup = self.clone(); // leave original report untouched by creating a fresh copy at each iteration
        dup.nums.remove(current); // remove num at index
        let is_safe = dup.check_safety(); // check safety with num removed
        if is_safe {
            // if safe without num at current position, set safe at original instance and exit recursion
            self.is_safe = true;
            return;
        }
        current += 1;
        // continue testing with removal at advanced position
        self.use_dampener(current);
    }
}

fn parse_input(input: String) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            Report::new(
                line.split_whitespace()
                    .map(|s| s.parse::<i8>().expect("Failed to parse u8."))
                    .collect::<Vec<i8>>(),
            )
        })
        .collect::<Vec<Report>>()
}

fn check_all(reports: &mut Vec<Report>) -> u16 {
    let mut num_safe: u16 = 0;

    for report in reports.iter_mut() {
        let is_safe = report.check_safety();
        if is_safe {
            num_safe += 1;
        }
    }

    num_safe
}

fn check_all_with_dampener(reports: &mut Vec<Report>) -> u16 {
    let mut num_safe: u16 = 0;

    for report in reports.iter_mut() {
        report.use_dampener(0);
        if report.is_safe {
            num_safe += 1;
        }
    }

    num_safe
}

mod part1 {
    use super::*;

    pub fn execute() {
        let input = read_input("../input/day2.txt");
        let mut reports = parse_input(input);
        let num_safe = check_all(&mut reports);

        println!("# SAFE: {}", num_safe);
    }
}

mod part2 {
    use super::*;

    pub fn execute() {
        let input = read_input("../input/day2.txt");
        let mut reports = parse_input(input);
        let num_safe = check_all_with_dampener(&mut reports);

        println!("# SAFE WITH DAMPENER: {}", num_safe);
    }
}

#[cfg(test)]
mod tests {
    use crate::Report;

    #[test]
    fn test_dampener() {
        let mut report = Report::new(vec![28, 30, 31, 33, 35, 42, 38]);
        report.use_dampener(0);
        println!("{:#?}", report);

        assert!(report.is_safe == true);
    }
}
