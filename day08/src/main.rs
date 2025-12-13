use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Coord3D {
    x: u64,
    y: u64,
    z: u64,
}

fn load_junction_boxes(filename: &str) -> Vec<Coord3D> {
    let mut coords = Vec::new();

    for line in utils::read_file(filename) {
        let splitted_line: Vec<_> = line.split(",").collect();
        if splitted_line.len() == 3 {
            let (x_str, y_str, z_str) = (splitted_line[0], splitted_line[1], splitted_line[2]);

            coords.push(Coord3D {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap(),
                z: z_str.parse().unwrap(),
            });
        }
    }

    coords
}

fn calculate_sizes(boxes: Vec<Coord3D>, number_of_shortest_connections: usize) -> usize {
    let mut res = 1;

    let mut distances: Vec<(Coord3D, Coord3D, u64)> = Vec::new();

    for (idx, first_box) in boxes.iter().enumerate() {
        for second_box in &boxes[idx + 1..] {
            let x = if first_box.x > second_box.x {
                first_box.x - second_box.x
            } else {
                second_box.x - first_box.x
            };
            let y = if first_box.y > second_box.y {
                first_box.y - second_box.y
            } else {
                second_box.y - first_box.y
            };
            let z = if first_box.z > second_box.z {
                first_box.z - second_box.z
            } else {
                second_box.z - first_box.z
            };
            distances.push((*first_box, *second_box, x * x + y * y + z * z))
        }
    }
    distances.sort_by_key(|x| x.2);
    println!("Distances: {:?}", distances.len());

    let mut circuits: Vec<BTreeSet<Coord3D>> = Vec::new();

    let mut iter = 0;
    for (first_box, second_box, _) in distances {
        iter += 1;
        let mut first_box_found: Option<usize> = None;
        let mut second_box_found: Option<usize> = None;

        for (i, coord_set) in circuits.iter().enumerate() {
            if coord_set.contains(&first_box) {
                first_box_found = Some(i);
            }
            if coord_set.contains(&second_box) {
                second_box_found = Some(i);
            }
        }

        match (first_box_found, second_box_found) {
            (Some(first_idx), Some(second_idx)) => {
                // found both, ignore or merge
                if first_idx != second_idx {
                    let (mut first_circuit, second_circuit) = if first_idx > second_idx {
                        (circuits.remove(first_idx), circuits.remove(second_idx))
                    } else {
                        (circuits.remove(second_idx), circuits.remove(first_idx))
                    };
                    first_circuit.extend(second_circuit);
                    circuits.push(first_circuit);
                } else {
                    // println!(
                    //     "Already connected {:?} {:?} in {}",
                    //     first_box, second_box, first_idx
                    // );
                }
            }
            (Some(first_idx), None) => {
                // found only first
                if let Some(circuit_containing_first) = circuits.get_mut(first_idx) {
                    circuit_containing_first.insert(second_box);
                } else {
                    todo!();
                }
            }
            (None, Some(second_idx)) => {
                // found only second
                if let Some(circuit_containing_second) = circuits.get_mut(second_idx) {
                    circuit_containing_second.insert(first_box);
                } else {
                    todo!();
                }
            }
            (None, None) => {
                // not found, need new circuit
                let mut coord_set = BTreeSet::new();
                coord_set.insert(first_box);
                coord_set.insert(second_box);
                circuits.push(coord_set);
            }
        }

        if iter >= number_of_shortest_connections {
            break;
        }
    }
    let mut circuit_lenghts: Vec<usize> = Vec::new();
    for c in &circuits {
        circuit_lenghts.push(c.len());
    }

    circuit_lenghts.sort();
    circuit_lenghts.reverse();
    for l in &circuit_lenghts[0..3] {
        res *= l;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_part1_working_with_test_input() {
        let boxes = load_junction_boxes("input_test");
        assert_eq!(calculate_sizes(boxes, 10), 40);
    }

    #[test]
    fn is_part2_working_with_test_input() {
        let boxes = load_junction_boxes("input_test");
    }
}

fn main() {
    let boxes = load_junction_boxes("day08/input");
    println!("Result: {}", calculate_sizes(boxes, 1000));
}
