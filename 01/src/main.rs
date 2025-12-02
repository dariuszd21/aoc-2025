use std::fs;

#[derive(Debug)]
enum Rotation {
    Left(u64),
    Right(u64),
}

pub fn read_file(filepath: &str) -> Vec<String> {
    let input_filepath = match std::env::current_dir() {
        Ok(cwd_filepath) => cwd_filepath.join(filepath),
        Err(_) => panic!("Cannot find current directory"),
    };
    println!("Input filepath: {}", input_filepath.display());
    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    file_content.split("\n").map(|s| s.to_string()).collect()
}

fn load_rotations(filepath: &str) -> Vec<Rotation> {
    let mut rotations = Vec::new();

    for line in read_file(filepath) {
        if line.len() > 1 {
            let mut rot = line.to_string();
            let num = rot.split_off(1);

            match num.parse::<u64>() {
                Ok(val) => match rot.as_str() {
                    "R" => rotations.push(Rotation::Right(val)),
                    "L" => rotations.push(Rotation::Left(val)),
                    &_ => todo!(),
                },
                Err(_) => todo!(),
            }
        }
    }

    rotations
}

fn perform_rotation(start: u64, rotation: &Rotation, higher_bound: u64) -> (u64, u64) {
    let lower_bound = 0;
    let mut zeros = 0;

    let mut result: u64 = start;

    match rotation {
        Rotation::Left(val) => {
            if *val > result {
                if result != 0 {
                    zeros += 1;
                }
                result = higher_bound - (val - result);
            } else {
                result -= val;
                if result == 0 {
                    zeros += 1;
                }
            }
        }
        Rotation::Right(val) => {
            if *val + result >= higher_bound {
                zeros += 1;
                result = lower_bound + ((result + val) - higher_bound);
            } else {
                result += val;
            }
        }
    }
    (zeros, result)
}

fn part_one(rotations: &Vec<Rotation>) {
    let start: u64 = 50;
    let higher_bound = 100;

    let mut res = start;
    let mut zeros = 0;
    for rotation in rotations {
        let rot = match rotation {
            Rotation::Left(val) => &Rotation::Left(val % higher_bound),
            Rotation::Right(val) => &Rotation::Right(val % higher_bound),
        };
        (_, res) = perform_rotation(res, rot, higher_bound);
        println!("{} {:?}", res, rotation);
        if res == 0 {
            zeros += 1;
        }
    }
    println!("Number of zeros: {}", zeros);
}

fn part_two(rotations: &Vec<Rotation>) {
    let start: u64 = 50;
    let higher_bound = 100;

    let mut res = start;
    let mut zeros = 0;
    for rotation in rotations {
        let rot = match rotation {
            Rotation::Left(val) => {
                let div = val / higher_bound;
                if div > 0 {
                    zeros += div;
                }
                &Rotation::Left(val % higher_bound)
            }
            Rotation::Right(val) => {
                let div = val / higher_bound;
                if div > 0 {
                    zeros += div;
                }

                &Rotation::Right(val % higher_bound)
            }
        };
        let zer;
        (zer, res) = perform_rotation(res, rot, higher_bound);
        zeros += zer;
        println!("{} {} {:?}", res, zer, rotation);
    }
    println!("Number of zeros: {}", zeros);
}

fn main() {
    let rotations = load_rotations("input_test");
    part_one(&rotations);
    part_two(&rotations);

    let rotations = load_rotations("input");
    part_one(&rotations);
    part_two(&rotations);
}
