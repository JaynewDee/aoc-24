#![allow(dead_code)]

fn main() {
    part1::execute();
    part2::execute();
}

fn read_input() -> String {
    std::fs::read_to_string("../input/day4.txt").expect("Failed to read input at provided path.")
}

#[derive(Debug)]
struct LetterMatrix {
    matrix: Vec<Vec<char>>,
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

impl LetterMatrix {
    fn find_by_coords(&self, coords: (usize, usize)) -> Option<char> {
        self.matrix
            .get(coords.1)
            .and_then(|row| row.get(coords.0).cloned())
    }

    fn check_direction(&self, start: (usize, usize), dir: (isize, isize)) -> bool {
        let mut coords = start;

        for &target_char in &['M', 'A', 'S'] {
            let new_coords = (
                coords.0.wrapping_add(dir.0 as usize),
                coords.1.wrapping_add(dir.1 as usize),
            );

            match self.find_by_coords(new_coords) {
                Some(c) if c == target_char => coords = new_coords,
                _ => return false,
            }
        }

        true
    }

    fn check_horizontal_right(&self, start: (usize, usize)) -> bool {
        self.check_direction(start, (1, 0))
    }

    fn check_horizontal_left(&self, start: (usize, usize)) -> bool {
        if start.0 < 3 {
            return false;
        }
        self.check_direction(start, (-1, 0))
    }

    fn check_vertical_up(&self, start: (usize, usize)) -> bool {
        if start.1 < 3 {
            return false;
        }
        self.check_direction(start, (0, -1))
    }

    fn check_vertical_down(&self, start: (usize, usize)) -> bool {
        self.check_direction(start, (0, 1))
    }

    fn check_diagonal_up_left(&self, start: (usize, usize)) -> bool {
        if start.0 < 3 || start.1 < 3 {
            return false;
        }
        self.check_direction(start, (-1, -1))
    }

    fn check_diagonal_up_right(&self, start: (usize, usize)) -> bool {
        if start.1 < 3 {
            return false;
        }
        self.check_direction(start, (1, -1))
    }

    fn check_diagonal_down_left(&self, start: (usize, usize)) -> bool {
        if start.0 < 3 {
            return false;
        }
        self.check_direction(start, (-1, 1))
    }

    fn check_diagonal_down_right(&self, start: (usize, usize)) -> bool {
        self.check_direction(start, (1, 1))
    }

    fn find_all_xmas(&self) -> u32 {
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

    // Top left to bottom right
    fn check_mas_first_diagonal(&self, start: (usize, usize)) -> bool {
        if start.0 < 1 || start.1 < 1 {
            return false;
        }
        if let Some(l) = self.find_by_coords((start.0 - 1, start.1 - 1)) {
            if l == 'M' {
                // check bottom right for 'S'
                if let Some(l) = self.find_by_coords((start.0 + 1, start.1 + 1)) {
                    if l == 'S' {
                        return true;
                    }
                }
            } else if l == 'S' {
                // check bottom right for 'M'
                if let Some(l) = self.find_by_coords((start.0 + 1, start.1 + 1)) {
                    if l == 'M' {
                        return true;
                    }
                }
            } else {
                return false;
            }
        };
        return false;
    }

    // Top right to bottom left
    fn check_mas_second_diagonal(&self, start: (usize, usize)) -> bool {
        if start.0 < 1 || start.1 < 1 {
            return false;
        }
        if let Some(l) = self.find_by_coords((start.0 + 1, start.1 - 1)) {
            if l == 'M' {
                // check bottom left for 'S'
                if let Some(l) = self.find_by_coords((start.0 - 1, start.1 + 1)) {
                    if l == 'S' {
                        return true;
                    }
                }
            } else if l == 'S' {
                // check bottom left for 'M'
                if let Some(l) = self.find_by_coords((start.0 - 1, start.1 + 1)) {
                    if l == 'M' {
                        return true;
                    }
                }
            } else {
                return false;
            }
        };
        return false;
    }

    fn find_all_mas_x(&self) -> u32 {
        let mut total = 0;

        for (y, row) in self.matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'A' {
                    if self.check_mas_first_diagonal((x, y))
                        && self.check_mas_second_diagonal((x, y))
                    {
                        total += 1;
                    }
                }
            }
        }

        total
    }
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

mod part2 {
    use super::*;

    pub fn execute() {
        let input = read_input();
        let letter_matrix = parse_input_matrix(&input);
        let total = letter_matrix.find_all_mas_x();

        println!("TOTAL MAS-X FOUND: {}", total);
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
