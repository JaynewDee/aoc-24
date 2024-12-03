const INPUT_PATH: &str = "../input/day1.txt";
const LIST_LEN: usize = 1000;

type U32Array = [u32; LIST_LEN];

const fn init_u32_array() -> U32Array {
    return [0; LIST_LEN];
}

fn main() {
    part1::execute();
    part2::execute();
}

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).expect("Unable to read input at provided path.")
}

fn parse_input(input: &str) -> (U32Array, U32Array) {
    let mut arr1 = init_u32_array();
    let mut arr2 = init_u32_array();

    input
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Failed to parse u32"))
        .enumerate()
        .for_each(|(i, n)| {
            // every other number is from the other list
            if i % 2 == 0 {
                arr1[i / 2] = n;
            } else {
                arr2[i / 2] = n;
            }
        });

    arr1.sort();
    arr2.sort();

    (arr1, arr2)
}

fn pair_nums(arrays: (U32Array, U32Array)) -> [(i32, i32); LIST_LEN] {
    let mut tuple_arr_ret: [(i32, i32); LIST_LEN] = [(0, 0); LIST_LEN];

    arrays
        .0
        .iter()
        .zip(arrays.1.iter())
        .enumerate()
        .for_each(|(i, (&a, &b))| tuple_arr_ret[i] = (a as i32, b as i32));

    tuple_arr_ret
}

/// Each tuple has structure (num_from_left, appearance_count_in_right)
fn get_occurrence_counts(lists: (U32Array, U32Array)) -> [(u32, u8); LIST_LEN] {
    let mut counts: [(u32, u8); LIST_LEN] = [(0, 0); LIST_LEN];

    lists.0.into_iter().enumerate().for_each(|(i, n1)| {
        let mut count: (u32, u8) = (n1, 0);
        for n2 in &lists.1 {
            // having sorted lists means that if our num from right list is greater, we can simply stop iterating
            if *n2 > n1 {
                break;
            }
            if *n2 == n1 {
                count.1 += 1;
            }
        }
        counts[i] = count;
    });

    counts
}

fn calculate_total_diff(pairs: [(i32, i32); LIST_LEN]) -> i32 {
    pairs.into_iter().map(|(n1, n2)| i32::abs(n1 - n2)).sum()
}

fn calculate_similarity_score(counts: [(u32, u8); LIST_LEN]) -> u32 {
    counts
        .into_iter()
        .fold(0, |acc, (n, c)| acc + (n * c as u32))
}

mod part1 {
    use super::*;

    pub fn execute() {
        let input_str = read_input(INPUT_PATH);
        let lists = parse_input(&input_str);
        let pairs = pair_nums(lists);
        let total_difference = calculate_total_diff(pairs);

        println!("PART 1 ANSWER: {total_difference}");
    }
}

mod part2 {
    use super::*;

    pub fn execute() {
        let input_str = read_input(INPUT_PATH);
        let lists = parse_input(&input_str);
        let counts = get_occurrence_counts(lists);
        let score = calculate_similarity_score(counts);

        println!("PART 2 ANSWER: {score}");
    }
}
