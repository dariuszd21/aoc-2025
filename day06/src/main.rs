#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

fn load_operations(filename: &str) -> (Vec<Op>, Vec<Vec<u64>>) {
    let mut ops = Vec::new();
    let mut values = Vec::new();

    let lines = utils::read_file(filename);

    for line in &lines[0..lines.len()] {
        let mut values_row = Vec::new();

        for val in line.split_ascii_whitespace() {
            match val.parse() {
                Ok(num_val) => values_row.push(num_val),
                Err(_) => (),
            }
            match val {
                "+" => ops.push(Op::Add),
                "*" => ops.push(Op::Multiply),
                _ => (),
            }
        }

        if !values_row.is_empty() {
            values.push(values_row);
        }
    }

    (ops, values)
}

fn calculate_result(ops: Vec<Op>, values: Vec<Vec<u64>>) -> u64 {
    let mut res = 0;

    for (idx, op) in ops.iter().enumerate() {
        match op {
            Op::Add => {
                let mut single_res = 0;
                for value_row in &values {
                    single_res += value_row[idx];
                }
                res += single_res;
            }
            Op::Multiply => {
                let mut single_res = 1;
                for value_row in &values {
                    single_res *= value_row[idx];
                }
                res += single_res;
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
        let (ops, values) = load_operations("input_test");
        assert_eq!(calculate_result(ops, values), 4277556);
    }

    #[test]
    fn is_part2_working_with_test_input() {}
}

fn main() {
    let (ops, values) = load_operations("day06/input");
    println!("Result: {}", calculate_result(ops, values));
}
