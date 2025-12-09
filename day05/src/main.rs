#[derive(Debug)]
struct Range {
    lower_bound: u64,
    upper_bound: u64,
}

fn find_fresh(ranges: Vec<Range>, ingredients: Vec<u64>) -> u64 {
    let mut res = 0;

    for ingredient in ingredients {
        for range in &ranges {
            if range.lower_bound <= ingredient && ingredient <= range.upper_bound {
                res += 1;
                break;
            }
        }
    }

    res
}

fn load_input(filename: &str) -> (Vec<Range>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in utils::read_file(filename) {
        if !line.is_empty() {
            let range: Vec<_> = line.split("-").collect();
            if range.len() == 2 {
                let lower_bound = match range[0].parse() {
                    Ok(val) => val,
                    Err(_) => todo!(),
                };

                let upper_bound = match range[1].parse() {
                    Ok(val) => val,
                    Err(_) => todo!(),
                };

                ranges.push(Range {
                    lower_bound: lower_bound,
                    upper_bound: upper_bound,
                });
            } else if range.len() == 1 {
                match range[0].parse() {
                    Ok(val) => ingredients.push(val),
                    Err(_) => todo!(),
                };
            }
        }
    }

    (ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_part1_working_with_test_input() {
        let (ranges, ingredients) = load_input("input_test");
        assert_eq!(find_fresh(ranges, ingredients), 3);
    }

    #[test]
    fn is_part2_working_with_test_input() {}
}

fn main() {
    let (ranges, ingredients) = load_input("day05/input");
    println!("Result: {}", find_fresh(ranges, ingredients));
}
