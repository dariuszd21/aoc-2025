#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Start,
    Splitter,
    Beam,
}

fn load_tiles(filename: &str) -> Vec<Vec<Tile>> {
    let mut tiles = Vec::new();

    for line in utils::read_file(filename) {
        let mut tiles_row = Vec::new();

        for c in line.chars() {
            match c {
                'S' => tiles_row.push(Tile::Start),
                '.' => tiles_row.push(Tile::Empty),
                '^' => tiles_row.push(Tile::Splitter),
                _ => (),
            }
        }

        if !tiles_row.is_empty() {
            tiles.push(tiles_row);
        }
    }

    tiles
}

fn calculate_splits(tiles: Vec<Vec<Tile>>) -> usize {
    let mut res = 0;

    let mut tiles_after_beam_passes: Vec<Vec<Tile>> = Vec::new();
    tiles_after_beam_passes.push(tiles[0].clone());

    for (i, tile_row) in tiles[1..].iter().enumerate() {
        let mut tiles_row_after_beam = tile_row.clone();

        for (j, tile) in tile_row.iter().enumerate() {
            match tile {
                Tile::Start => (),
                Tile::Splitter => {
                    if tiles_after_beam_passes[i][j] == Tile::Beam {
                        if j > 0 {
                            tiles_row_after_beam[j - 1] = Tile::Beam;
                        }
                        if j < tile_row.len() {
                            tiles_row_after_beam[j + 1] = Tile::Beam;
                        }
                    }
                }
                Tile::Empty => {
                    if tiles_after_beam_passes[i][j] == Tile::Beam
                        || tiles_after_beam_passes[i][j] == Tile::Start
                    {
                        tiles_row_after_beam[j] = Tile::Beam;
                    }
                }
                Tile::Beam => (),
            }
        }
        tiles_after_beam_passes.push(tiles_row_after_beam);
    }

    tiles_after_beam_passes.reverse();
    for (i, tile_row) in tiles_after_beam_passes[1..tiles_after_beam_passes.len()]
        .iter()
        .enumerate()
    {
        for (j, tile) in tile_row.iter().enumerate() {
            match tile {
                Tile::Empty => (),
                Tile::Start => (),
                Tile::Splitter => (),
                Tile::Beam => {
                    if tiles_after_beam_passes[i][j] != Tile::Beam {
                        res += 1;
                    }
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
    fn is_part1_working_with_test_input() {
        let tiles = load_tiles("input_test");
        assert_eq!(calculate_splits(tiles), 21);
    }

    #[test]
    fn is_part2_working_with_test_input() {
        let tiles = load_tiles("input_test");
    }
}
fn main() {
    let tiles = load_tiles("day07/input");
    println!("Result {}", calculate_splits(tiles));
}
