fn main() {
    part1::execute();
    part2::execute();
}

fn read_input(path: &str) -> String {
    let mut input = std::fs::read_to_string(path).expect("Failed to read input at provided path.");
    input.retain(|c| c != '\n' && c != '\r');
    input
}

struct Instruction(u16, u16);

fn parse_input(input: &mut String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    let mul_indices: Vec<_> = input.match_indices("mul(").map(|v| v.0).collect();

    for i in mul_indices.iter() {
        let window = &mut input[*i + 4..*i + 12]
            .split("")
            .filter(|v| *v != "")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let mut window_iter = window.iter();
        let mut pred1 = String::new();
        let mut pred2 = String::new();
        let mut passed_comma = false;
        let mut instruction_valid = true;

        while let Some(v) = window_iter.next() {
            if *v == ")" {
                break;
            }

            if *v == "," {
                passed_comma = true;
                continue;
            }

            match v.parse::<u8>() {
                Ok(n) => {
                    if passed_comma {
                        pred2.push_str(&n.to_string());
                    } else {
                        pred1.push_str(&n.to_string());
                    }
                }
                _ => {
                    instruction_valid = false;
                    break;
                }
            }
        }

        if instruction_valid {
            let pred1 = pred1.parse::<u16>();
            let pred2 = pred2.parse::<u16>();
            if pred1.is_ok() && pred2.is_ok() {
                let instruction = Instruction(pred1.unwrap(), pred2.unwrap());

                instructions.push(instruction);
            }
        }
    }

    instructions
}

fn aggregate_mult_instructions(instructions: Vec<Instruction>) -> u32 {
    instructions
        .into_iter()
        .fold(0u32, |acc, v| acc + (v.0 as u32 * v.1 as u32))
}

fn remove_disabled_zones(input: String) -> String {
    let mut ret = input;
    while let Some(start) = ret.find("don't()") {
        if let Some(end) = ret[start..].find("do()") {
            let end = start + end;
            ret.replace_range(start..end + 4, "X");
        } else {
            break;
        }
    }

    ret
}

mod part1 {
    use super::*;

    pub fn execute() {
        let mut input = read_input("../input/day3.txt");
        let instructions = parse_input(&mut input);
        let total = aggregate_mult_instructions(instructions);
        println!("PART 1 TOTAL: {}", &total);
    }
}

mod part2 {
    use super::*;

    pub fn execute() {
        let input = read_input("../input/day3.txt");
        let mut with_disabled_zones = remove_disabled_zones(input);
        let instructions = parse_input(&mut with_disabled_zones);
        let total = aggregate_mult_instructions(instructions);
        println!("PART 2 TOTAL: {}", total);
    }
}
