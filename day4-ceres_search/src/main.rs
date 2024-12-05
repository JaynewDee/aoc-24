#![allow(dead_code)]

fn main() {
    part1::execute();
}

fn read_input() -> String {
    std::fs::read_to_string("../input/day4.txt").expect("Failed to read input at provided path.")
}

#[derive(Debug)]
struct LetterMatrix {
    matrix: Vec<Vec<char>>,
}

impl LetterMatrix {
    pub fn find_by_coords(&self, coords: (usize, usize)) -> Option<char> {
        if let Some(row) = self.matrix.get(coords.1) {
            if let Some(item) = row.get(coords.0) {
                return Some(*item);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn check_horizontal_right(&self, start: (usize, usize)) -> bool {
        if let Some(m) = self.find_by_coords((start.0 + 1, start.1)) {
            if m == 'M' {
                if let Some(a) = self.find_by_coords((start.0 + 2, start.1)) {
                    if a == 'A' {
                        if let Some(s) = self.find_by_coords((start.0 + 3, start.1)) {
                            if s == 'S' {
                                return true;
                            }
                        }
                    }
                };
            }
        }
        return false;
    }

    pub fn check_horizontal_left(&self, start: (usize, usize)) -> bool {
        if start.0 < 3 {
            return false;
        }
        if let Some(m) = self.find_by_coords((start.0 - 1, start.1)) {
            if m == 'M' {
                if let Some(a) = self.find_by_coords((start.0 - 2, start.1)) {
                    if a == 'A' {
                        if let Some(s) = self.find_by_coords((start.0 - 3, start.1)) {
                            if s == 'S' {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn check_vertical_up(&self, start: (usize, usize)) -> bool {
        if start.1 < 3 {
            return false;
        }
        if let Some(m) = self.find_by_coords((start.0, start.1 - 1)) {
            if m == 'M' {
                if let Some(a) = self.find_by_coords((start.0, start.1 - 2)) {
                    if a == 'A' {
                        if let Some(s) = self.find_by_coords((start.0, start.1 - 3)) {
                            if s == 'S' {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn check_vertical_down(&self, start: (usize, usize)) -> bool {
        if let Some(m) = self.find_by_coords((start.0, start.1 + 1)) {
            if m == 'M' {
                if let Some(a) = self.find_by_coords((start.0, start.1 + 2)) {
                    if a == 'A' {
                        if let Some(s) = self.find_by_coords((start.0, start.1 + 3)) {
                            if s == 'S' {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn check_diagonal_up_left(&self, start: (usize, usize)) -> bool {
        if start.0 < 3 || start.1 < 3 {
            return false;
        }
        if let Some(m) = self.find_by_coords((start.0 - 1, start.1 - 1)) {
            if m == 'M' {
                if let Some(a) = self.find_by_coords((start.0 - 2, start.1 - 2)) {
                    if a == 'A' {
                        if let Some(s) = self.find_by_coords((start.0 - 3, start.1 - 3)) {
                            if s == 'S' {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn check_diagonal_up_right(&self, start: (usize, usize)) -> bool {
        if start.1 < 3 {
            return false;
        }
        if let Some(m) = self.find_by_coords((start.0 + 1, start.1 - 1)) {
            if m == 'M' {
                if let Some(a) = self.find_by_coords((start.0 + 2, start.1 - 2)) {
                    if a == 'A' {
                        if let Some(s) = self.find_by_coords((start.0 + 3, start.1 - 3)) {
                            if s == 'S' {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn check_diagonal_down_left(&self, start: (usize, usize)) -> bool {
        if start.0 < 3 {
            return false;
        }
        if let Some(m) = self.find_by_coords((start.0 - 1, start.1 + 1)) {
            if m == 'M' {
                if let Some(a) = self.find_by_coords((start.0 - 2, start.1 + 2)) {
                    if a == 'A' {
                        if let Some(s) = self.find_by_coords((start.0 - 3, start.1 + 3)) {
                            if s == 'S' {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn check_diagonal_down_right(&self, start: (usize, usize)) -> bool {
        if let Some(m) = self.find_by_coords((start.0 + 1, start.1 + 1)) {
            if m == 'M' {
                if let Some(a) = self.find_by_coords((start.0 + 2, start.1 + 2)) {
                    if a == 'A' {
                        if let Some(s) = self.find_by_coords((start.0 + 3, start.1 + 3)) {
                            if s == 'S' {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn find_all_xmas(&self) -> u32 {
        let mut total = 0;

        for (y, row) in self.matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'X' {
                    if self.check_horizontal_right((x, y)) {
                        total += 1;
                    }

                    if self.check_horizontal_left((x, y)) {
                        total += 1;
                    }

                    if self.check_vertical_up((x, y)) {
                        total += 1;
                    }

                    if self.check_vertical_down((x, y)) {
                        total += 1;
                    }

                    if self.check_diagonal_up_left((x, y)) {
                        total += 1;
                    }

                    if self.check_diagonal_up_right((x, y)) {
                        total += 1;
                    }

                    if self.check_diagonal_down_left((x, y)) {
                        total += 1;
                    }

                    if self.check_diagonal_down_right((x, y)) {
                        total += 1;
                    }
                }
            }
        }

        total
    }
}

fn parse_input_matrix(input: &String) -> LetterMatrix {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        matrix.push(row);
    }

    LetterMatrix { matrix }
}

mod part1 {
    use super::*;

    pub fn execute() {
        let input = read_input();
        let letter_matrix = parse_input_matrix(&input);
        let total = letter_matrix.find_all_xmas();

        println!("TOTAL XMAS FOUND: {}", total);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_by_coords() {
        let input = read_input();
        let letter_matrix = parse_input_matrix(&input);

        let c = letter_matrix.find_by_coords((132, 138));
        assert!(c.is_some() && c.unwrap() == 'X');

        let c = letter_matrix.find_by_coords((139, 139));
        assert!(c.is_some() && c.unwrap() == 'S');

        let c = letter_matrix.find_by_coords((0, 0));
        assert!(c.is_some() && c.unwrap() == 'X');

        let c = letter_matrix.find_by_coords((75, 138));
        assert!(c.is_some() && c.unwrap() == 'M');
    }
}
