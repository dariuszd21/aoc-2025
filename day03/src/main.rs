use utils::{self, read_file};

fn read_banks(filename: &str) -> Vec<Vec<u64>> {
    let mut banks = Vec::new();

    for line in read_file(filename) {
        let mut bank = Vec::new();
        for c in line.chars() {
            match c.to_string().parse() {
                Ok(value) => bank.push(value),
                Err(_) => panic!("Unknown battery"),
            }
        }

        if !bank.is_empty() {
            banks.push(bank);
        }
    }

    banks
}

fn find_highest_joltage(banks: Vec<Vec<u64>>) -> u64 {
    let mut res = 0;

    for bank in banks {
        let slice = &bank[0..(bank.len() - 1)];
        if let Some(max) = slice.iter().max() {
            if let Some(index) = slice.iter().position(|x| x == max) {
                if let Some(max_second) = &bank[index + 1..].iter().max() {
                    let num_str = format!("{}{}", max, max_second);
                    match num_str.parse::<u64>() {
                        Ok(val) => res += val,
                        Err(_) => panic!("Cannot parse {}", num_str),
                    }
                }
            } else {
                panic!("Cannot find max {}", max);
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_part1_working_with_test_input() {
        let banks = read_banks("input_test");
        assert_eq!(find_highest_joltage(banks), 357);
    }

    #[test]
    fn is_part2_working_with_test_input() {}
}
fn main() {
    let banks = read_banks("day03/input");
    println!("Total output is: {}", find_highest_joltage(banks));
}
