fn main() {
    let input = read_input();
    let mut parsed = parse_input(input);

    swap_from_end(&mut parsed);

    let mut total = 0u64;
    let mut pos = 0u64;

    parsed.iter().for_each(|n| {
        if let Some(id) = n {
            total += id * pos;
            pos += 1;
        }
    });

    println!("{total}");
}

fn read_input() -> String {
    std::fs::read_to_string("../input/day9.txt").expect("Failed to read input at provided path.")
}

fn parse_input(input: String) -> Vec<Option<u64>> {
    let mut ret: Vec<Option<u64>> = Vec::new();

    let mut current_id = 0 as u64;

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            if let Some(num_blocks) = c.to_digit(10) {
                for _ in 0..num_blocks {
                    ret.push(Some(current_id));
                }
            }
        } else {
            if let Some(num_spaces) = c.to_digit(10) {
                for _ in 0..num_spaces {
                    ret.push(None);
                }
                current_id += 1;
            }
        }
    }

    ret
}

fn swap_from_end(vec: &mut Vec<Option<u64>>) {
    let mut none_index = 0; // Pointer to the next None value
    let mut some_index = vec.len() - 1; // Pointer to the last Some value

    while none_index < some_index {
        // Move none_index to the first None value
        while none_index < vec.len() && vec[none_index].is_some() {
            none_index += 1;
        }

        // Move some_index to the last Some value
        while some_index > none_index && vec[some_index].is_none() {
            some_index -= 1;
        }

        // Swap if a None and a Some are found in the correct positions
        if none_index < some_index {
            vec.swap(none_index, some_index);
        }
    }
}
