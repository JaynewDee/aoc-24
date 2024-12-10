/* X & Y coordinates are designed as follows:

- topmost leftmost character is 0,0 (x, y)
- x coordinate increases as we move right
- y coordinate increases as we move down
*/

fn main() {
    let input = read_input();
    let mut map = Map::from(input);

    map.traverse();

    let visited_count = map.count_visited();
    println!("{}", visited_count);

    // println!("{:#?}", map.locations);
}

fn read_input() -> String {
    std::fs::read_to_string("../input/day6.txt").expect("Failed to read input at provided path.")
}

#[derive(Debug)]
struct Location {
    x: usize,
    y: usize,
    c: Option<char>,
}

// #[default] None
#[derive(Default, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    #[default]
    None,
}

#[derive(Default, Debug)]
struct Guard {
    facing: Direction,
    position: (usize, usize),
}

impl Guard {
    fn turn(&mut self) {
        use Direction::*;
        match self.facing {
            Up => self.facing = Right,
            Right => self.facing = Down,
            Down => self.facing = Left,
            Left => self.facing = Up,
            _ => {}
        }
    }

    fn advance(&mut self) {
        use Direction::*;
        match self.facing {
            Up => self.position = (self.position.0, self.position.1 - 1),
            Right => self.position = (self.position.0 + 1, self.position.1),
            Down => self.position = (self.position.0, self.position.1 + 1),
            Left => self.position = (self.position.0 - 1, self.position.1),
            _ => {}
        }
    }
}

#[derive(Default, Debug)]
struct Map {
    locations: Vec<Location>,
    guard: Guard,
    dimensions: (usize, usize), // (width, height)
}

impl Map {
    fn traverse(&mut self) {
        use Direction::*;
        /* Loop
           1) Attempt to move the guard in the direction it's facing
           2) If successful/path clear, store 'X' as char at visited location
           3) If blocked, turn right 90deg
           4) If at and facing edge, break
        */
        let (max_x, max_y) = (self.dimensions.0 - 1, self.dimensions.1 - 1);
        loop {
            match self.guard.facing {
                Up => {
                    if self.guard.position.1 == 0 {
                        break;
                    }
                    let (target_x, target_y) = (self.guard.position.0, self.guard.position.1 - 1);
                    if self.is_clear_at_coord(target_x, target_y) {
                        self.guard.advance();
                        self.mark_target(target_x, target_y);
                    } else {
                        self.guard.turn();
                    }
                }
                Right => {
                    if self.guard.position.0 == max_x {
                        break;
                    }
                    let (target_x, target_y) = (self.guard.position.0 + 1, self.guard.position.1);
                    if self.is_clear_at_coord(target_x, target_y) {
                        self.guard.advance();
                        self.mark_target(target_x, target_y);
                    } else {
                        self.guard.turn();
                    }
                }
                Down => {
                    if self.guard.position.1 == max_y {
                        break;
                    }
                    let (target_x, target_y) = (self.guard.position.0, self.guard.position.1 + 1);
                    if self.is_clear_at_coord(target_x, target_y) {
                        self.guard.advance();
                        self.mark_target(target_x, target_y);
                    } else {
                        self.guard.turn();
                    }
                }
                Left => {
                    if self.guard.position.0 == 0 {
                        break;
                    }
                    let (target_x, target_y) = (self.guard.position.0 - 1, self.guard.position.1);
                    if self.is_clear_at_coord(target_x, target_y) {
                        self.guard.advance();
                        self.mark_target(target_x, target_y);
                    } else {
                        self.guard.turn();
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn is_clear_at_coord(&self, x: usize, y: usize) -> bool {
        self.locations
            .iter()
            .find(|el| el.x == x && el.y == y)
            .and_then(|location| location.c)
            .map_or(true, |c| c != '#')
    }

    fn mark_target(&mut self, x: usize, y: usize) {
        if let Some(location) = self
            .locations
            .iter_mut()
            .find(|loc| loc.x == x && loc.y == y)
        {
            location.c = Some('X');
        }
    }

    fn count_visited(&self) -> u32 {
        self.locations.iter().fold(0, |acc, v| {
            if v.c.is_some_and(|c| c == 'X') {
                acc + 1
            } else {
                acc
            }
        })

    }
}

impl From<String> for Map {
    fn from(value: String) -> Self {
        let mut map = Map::default();
        let mut locations: Vec<Location> = Vec::new();
        let mut dimensions: (usize, usize) = (0, 0);

        for (y, line) in value.lines().enumerate() {
            dimensions.1 = y + 1; // Update height to the current line number + 1
            if dimensions.0 == 0 {
                dimensions.0 = line.len(); // Set width using the first line
            }

            for (x, c) in line.chars().enumerate() {
                let is_guard = matches!(c, '^' | '<' | '>' | 'v');
                if is_guard {
                    let mut guard = Guard::default();
                    guard.position = (x, y);
                    match c {
                        '^' => guard.facing = Direction::Up,
                        '>' => guard.facing = Direction::Right,
                        'v' => guard.facing = Direction::Down,
                        '<' => guard.facing = Direction::Left,
                        _ => {}
                    }

                    map.guard = guard;
                }

                let l = Location {
                    x,
                    y,
                    c: match c {
                        '#' => Some(c),
                        _ if is_guard => Some('X'),
                        _ => None,
                    },
                };

                locations.push(l);
            }
        }

        map.locations = locations;
        map.dimensions = dimensions;
        map
    }
}
