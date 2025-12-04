use utils;

#[derive(Debug)]
struct Range {
    lower: u64,
    upper: u64,
}

fn read_ranges(filename: &str) -> Vec<Range> {
    let lines = utils::read_file(filename);
    let mut ranges = Vec::new();
    for line in lines {
        for range in line.split(",") {
            let bounds: Vec<_> = range.split("-").collect();
            if bounds.len() == 2 {
                let (lower, upper) = (bounds[0], bounds[1]);
                ranges.push(Range {
                    lower: lower.parse().unwrap(),
                    upper: upper.parse().unwrap(),
                });
            }
        }
    }

    ranges
}

fn find_palindrom_numbers_part1(ranges: Vec<Range>) -> u64 {
    let mut res = 0;

    for range in ranges {
        for i in range.lower..=range.upper {
            let string_num = i.to_string();
            let num_len = string_num.len();
            if num_len % 2 == 0 {
                if string_num[0..num_len / 2] == string_num[num_len / 2..] {
                    res += i;
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_working_with_test_input() {
        let ranges = read_ranges("input_test");
        assert_eq!(find_palindrom_numbers_part1(ranges), 1227775554);
    }
}

fn main() {
    let ranges = read_ranges("day02/input");
    println!("Result: {}", find_palindrom_numbers_part1(ranges));
}
