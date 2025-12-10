#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

fn load_operations_and_values_part1(filename: &str) -> (Vec<Op>, Vec<Vec<u64>>) {
    let mut ops = Vec::new();
    let mut values = Vec::new();

    let lines = utils::read_file(filename);

    for line in &lines {
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

fn load_operations_and_values_part2(filename: &str) -> (Vec<Op>, Vec<Vec<u64>>) {
    let mut ops = Vec::new();
    let mut values = Vec::new();

    let mut lines = utils::read_file(filename);

    let mut op_idx = 0;

    for (idx, line) in lines.iter().enumerate() {
        for val in line.split_ascii_whitespace() {
            op_idx = idx;
            match val {
                "+" => ops.push(Op::Add),
                "*" => ops.push(Op::Multiply),
                _ => (),
            }
        }
    }
    lines = (&lines[0..op_idx]).to_vec();

    let mut operator_values = Vec::new();

    if let Some(first_line) = lines.first() {
        for (idx, _) in first_line.char_indices() {
            let mut num = String::new();
            for line in &lines {
                if line.is_empty() {
                    continue;
                }
                num += &line[idx..idx + 1];
            }
            num = num.trim().to_string();
            if num.is_empty() {
                if !operator_values.is_empty() {
                    values.push(operator_values);
                }
                operator_values = Vec::new();
            }
            match num.parse() {
                Ok(val) => operator_values.push(val),
                Err(_) => (),
            }
        }
    }
    if !operator_values.is_empty() {
        values.push(operator_values);
    }

    (ops, values)
}

fn calculate_result_part2(ops: Vec<Op>, values: Vec<Vec<u64>>) -> u64 {
    let mut res = 0;

    for (idx, op) in ops.iter().enumerate() {
        match op {
            Op::Add => {
                let mut single_res = 0;
                for value in &values[idx] {
                    single_res += value;
                }
                res += single_res;
            }
            Op::Multiply => {
                let mut single_res = 1;
                for value in &values[idx] {
                    single_res *= value;
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
        let (ops, values) = load_operations_and_values_part1("input_test");
        assert_eq!(calculate_result(ops, values), 4277556);
    }

    #[test]
    fn is_part2_working_with_test_input() {
        let (ops, values) = load_operations_and_values_part2("input_test");
        assert_eq!(calculate_result_part2(ops, values), 3263827);
    }
}

fn main() {
    let (ops, values) = load_operations_and_values_part1("day06/input");
    println!("Result: {}", calculate_result(ops, values));

    let (ops, values) = load_operations_and_values_part2("day06/input");
    println!("Result: {}", calculate_result_part2(ops, values));
}
