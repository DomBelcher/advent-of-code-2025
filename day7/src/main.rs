use std::{collections::HashSet, collections::HashMap, fs};

const FILENAME: &str = "./input.txt";

const START_CHAR: char = 'S';
const SPLITTER_CHAR: char = '^';
const EMPTY_CHAR: char = '.';

fn main() {
   let (
    (width, height),
    start_coords,
    splitter_coords
   ) = parse_input();

    let mut tachyon_beams = HashMap::new();
    let mut timeline_beams = vec![];
    tachyon_beams.insert(start_coords, 1_i64);
    timeline_beams.push(start_coords);

    let mut hit_splitters = HashSet::new();

    let mut layer = 0;
    while layer < height {
        println!("Layer {}", layer);

        let mut new_beams = HashMap::new();
        for (beam, count) in tachyon_beams {
            let next_coords = (beam.0, beam.1 + 1);
            if splitter_coords.contains(&next_coords) {
                hit_splitters.insert(next_coords);

                if next_coords.0 != 0 {
                    let nc = (next_coords.0 - 1, next_coords.1);
                    if new_beams.contains_key(&nc) {
                        let current_count = new_beams.get(&nc).unwrap();
                        new_beams.insert(nc, *current_count + count);
                    } else {
                        new_beams.insert(nc, count);
                    }
                }

                if next_coords.0 != (width - 1) {
                    let nc = (next_coords.0 + 1, next_coords.1);
                    if new_beams.contains_key(&nc) {
                        let current_count = new_beams.get(&nc).unwrap();
                        new_beams.insert(nc, *current_count + count);
                    } else {
                        new_beams.insert(nc, count);
                    }
                }
            } else {
                if new_beams.contains_key(&next_coords) {
                    let current_count = new_beams.get(&next_coords).unwrap();
                    new_beams.insert(next_coords, *current_count + count);
                } else {
                    new_beams.insert(next_coords, count);
                }
            }
        }
        tachyon_beams = new_beams;
        layer += 1;
    }

    println!("Hit {} splitters", hit_splitters.len());
    let mut total = 0;
    for (_beam, count) in tachyon_beams {
        total += count;
    }
    println!("Total timelines: {}", total);
}

fn parse_input () -> ((usize, usize), (usize, usize), HashSet<(usize, usize)>) {
    let mut start_coords = None;
    let mut splitter_coords = HashSet::new();

    let mut width: usize = 0;
    let mut height: usize = 0;

    for (y, line) in fs::read_to_string(FILENAME).unwrap().lines().enumerate() {
        width = line.len();
        height += 1;

        for (x, c) in line.chars().enumerate() {
            if c == START_CHAR {
                start_coords = Some((x, y));
            } else if c == SPLITTER_CHAR {
                splitter_coords.insert((x, y));
            }
        }
    }

    return (
        (width, height),
        start_coords.unwrap(),
        splitter_coords
    )
}
