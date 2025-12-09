use utils;

#[derive(Debug)]
enum Tile {
    Empty,
    Roll,
}

fn load_tiles(filename: &str) -> Vec<Vec<Tile>> {
    let mut tiles = Vec::new();

    for line in utils::read_file(filename) {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(Tile::Empty),
                '@' => row.push(Tile::Roll),
                _ => todo!(),
            }
        }
        if !row.is_empty() {
            tiles.push(row);
        }
    }

    tiles
}

fn is_movable(i: usize, j: usize, num_cols: usize, tile: &Tile, tiles: &Vec<Vec<Tile>>) -> bool {
    match tile {
        Tile::Empty => false,
        Tile::Roll => {
            let mut rolls = 0;
            let left_bound = if j > 0 { j - 1 } else { 0 };
            let right_bound = if j < num_cols - 2 {
                j + 1
            } else {
                num_cols - 1
            };
            let lower_bound = if i > 0 { i - 1 } else { 0 };
            let upper_bound = if i < tiles.len() - 2 {
                i + 1
            } else {
                tiles.len() - 1
            };
            for i_nb in lower_bound..=upper_bound {
                for j_nb in left_bound..=right_bound {
                    if i_nb == i && j_nb == j {
                        continue;
                    }
                    if let Some(other_row) = tiles.get(i_nb) {
                        if let Some(tile) = other_row.get(j_nb) {
                            match tile {
                                Tile::Empty => (),
                                Tile::Roll => rolls += 1,
                            }
                        }
                    }
                }
            }
            rolls < 4
        }
    }
}

fn find_accessible(tiles: Vec<Vec<Tile>>) -> u64 {
    let mut res = 0;

    for (i, row) in tiles.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if is_movable(i, j, row.len(), &tile, &tiles) {
                res += 1;
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
        let tiles = load_tiles("input_test");
        assert_eq!(find_accessible(tiles), 13);
    }

    #[test]
    fn is_part2_working_with_test_input() {}
}

fn main() {
    let tiles = load_tiles("day04/input");
    println!("Result: {}", find_accessible(tiles));
}
