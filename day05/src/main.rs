#[derive(Debug, Clone, Copy)]

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

fn combine_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut sorted_ranges: Vec<_> = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.lower_bound.cmp(&b.lower_bound));

    // invert logic - add ranges one-by-one by either extending the previous ones or creating a new
    // entry

    let mut combined_ranges: Vec<Range> = Vec::new();
    for range in &sorted_ranges {
        let mut insert_new = true;
        // println!("Analysing: {:?}", range);
        for combined_range in &mut combined_ranges {
            // println!("Combined: {:?}", combined_range);
            if combined_range.lower_bound <= range.lower_bound
                && range.lower_bound <= combined_range.upper_bound
            {
                let max_upper = if range.upper_bound > combined_range.upper_bound {
                    range.upper_bound
                } else {
                    combined_range.upper_bound
                };

                *combined_range = Range {
                    lower_bound: combined_range.lower_bound,
                    upper_bound: max_upper,
                };
                // println!("Found {:?}", combined_range);
                insert_new = false;
                break;
            }
        }
        if insert_new {
            // println!("Inserting: {:?}", range);
            combined_ranges.push(range.clone());
        }
    }

    combined_ranges
}

fn count_fresh_ids(ranges: Vec<Range>) -> u64 {
    let mut res = 0;

    for range in combine_ranges(ranges) {
        res += (range.upper_bound - range.lower_bound) + 1;
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
    fn is_part2_working_with_test_input() {
        let (ranges, _) = load_input("input_test");
        assert_eq!(count_fresh_ids(ranges), 14);
    }
}

fn main() {
    let (ranges, ingredients) = load_input("day05/input");
    println!("Result: {}", find_fresh(ranges, ingredients));
    let (ranges, _) = load_input("day05/input");
    println!("Result: {}", count_fresh_ids(ranges));
}
