use std::cmp::{max, min};
use std::i32;
use std::{collections::HashSet, fs, time::Instant};
use std::iter::FromIterator;

const FILENAME: &str = "./input.txt";

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let start = Instant::now();

    let red_tiles = parse_input();
    println!("Input parsed in {}ms", start.elapsed().as_millis());


    let mut extrema = Extrema::new();

    let n_red_tiles = red_tiles.len();

    let mut max_area = 0;
    
    for i in 0..n_red_tiles {
        let rt1 = red_tiles[i];

        for j in (i+1)..n_red_tiles {
            let rt2 = red_tiles[j];

            let area = ((rt2.0 - rt1.0).abs() as i64 + 1) * ((rt2.1 - rt1.1).abs() as i64 + 1);
            if area > max_area {
                max_area = area;
            }
        }

        if rt1.0 < extrema.min_x {
            extrema.min_x = rt1.0;
        }
        if rt1.0 > extrema.max_x {
            extrema.max_x = rt1.0;
        }
        if rt1.1 < extrema.min_y {
            extrema.min_y = rt1.1;
        }
        if rt1.1 > extrema.max_y {
            extrema.max_y = rt1.1;
        }
    }

    println!("Max area: {}", max_area);

    println!("Part 1 ran in {}ms", start.elapsed().as_millis());


    let mut boundary = vec![];

    for i in 0..n_red_tiles {
        let v0 = red_tiles[i];
        let v1 = red_tiles[(i + 1) % n_red_tiles];
        // let v2 = red_tiles[(i + 2) % n_red_tiles];
        // let e2 = (v2.0 - v1.0, v2.1 - v1.1);

        boundary.append(&mut calc_edge(v0, v1));
    }

    println!("Calculted boundary in {}ms", start.elapsed().as_millis());
    // println!("Boundary:");
    // println!("{:?}", boundary);
    
    let boundary_set: HashSet<(i32, i32)> = HashSet::from_iter(boundary.iter().cloned());

    let mut border_point = (extrema.max_x + 1, extrema.max_y);;
    let mut direction_indicator = 1;

    loop {
        let next_border_point = next_point(border_point, (-1, 0));
        if boundary_set.contains(&next_border_point) {
            break;
        }
        border_point = next_border_point;
    }

    let start_point = border_point;

    println!("Extrema:");
    println!("{:?}", extrema);

    // let exterior_point = (extrema.max_x + 1, extrema.max_y);

    // let mut border_point = exterior_point;

    let mut border = vec![];
    border.push(start_point);

    println!("starting at: {:?}", start_point);

    loop {
        let dir = DIRECTIONS[direction_indicator];
        let rotated_dir = DIRECTIONS[(direction_indicator + 1) % 4];

        // println!("Border point is: {:?}", border_point);
        // next point in same direction
        let next_border_point = next_point(border_point, dir);
        // println!("Next border point is: {:?}", next_border_point);
        // point to the left of this direction
        let possible_boundary_point = next_point(border_point, rotated_dir);
        // println!("Next lefthand point is: {:?}", possible_boundary_point);

        if next_border_point == start_point {
            // finished
            break
        }

        if boundary_set.contains(&next_border_point) {
            // println!("Next border point is on the boundary");
            // next point along this direction is in the boundary, rotate 90 degrees clockwise
            border.push(border_point);

            direction_indicator = (direction_indicator + 3) % 4;
            continue
        }

        if !boundary_set.contains(&possible_boundary_point) {
            // println!("Next lefthand point is not on the boundary");
            // point to the left is not in the boundary, rotate 90 degrees anticlockwise
            border.push(border_point);
            direction_indicator = (direction_indicator + 1) % 4;
            border_point = possible_boundary_point;
            continue
        }

        // still tracking the border, continue
        border_point = next_border_point;
    }

    println!("Calculated border in {}ms", start.elapsed().as_millis());
    // println!("{:?}", border);

    let n_border_coords = border.len();

    let mut part_2_max_area = 0;
    let mut best_rect = Extrema::new();

    for i in 0..n_red_tiles {
        let rt1 = red_tiles[i];

        for j in (i+1)..n_red_tiles {
            let rt2 = red_tiles[j];

            let rect = Extrema::from_points(rt1, rt2);

            let mut all_coloured_tiles = true;

            for border_idx in 0..n_border_coords {
                let border_v1 = border[border_idx];
                let border_v2 = border[(border_idx + 1) % n_border_coords];

                if line_crosses_rect((border_v1, border_v2), &rect) || within_extrema(border_v1, &rect) {
                    all_coloured_tiles = false;
                    break;
                }
            }

            if all_coloured_tiles {

                let area = ((rt2.0 - rt1.0).abs() as i64 + 1) * ((rt2.1 - rt1.1).abs() as i64 + 1);

                if area > part_2_max_area {
                    best_rect = rect;
                    part_2_max_area = area;
                }
            }
        }
    }

    println!("Part 2 max area: {}", part_2_max_area);
    println!("Largest rectangle is {:?}", best_rect);

    println!("Part 2 ran in {}ms", start.elapsed().as_millis());
}

fn line_crosses_rect (line: ((i32, i32), (i32, i32)), rect: &Extrema) -> bool {
    let c1 = line.0;
    let c2 = line.1;

    if c1.0 == c2.0 {
        // x is constant
        if c1.0 <= rect.min_x || rect.max_x <= c1.0 {
            // x doesn't fall within rect
            return false
        }
        if c1.1 <= rect.min_y && rect.max_y <= c2.1 {
            // y crosses rect
            return true
        } 
         if c2.1 <= rect.min_y && rect.max_y <= c1.1 {
            // y crosses rect
            return true
        }
        return false
    } else {
        // y is constant
        if c1.1 <= rect.min_y || rect.max_y <= c1.1 {
            // y doesn't fall within rect
            return false
        }
        if c1.0 <= rect.min_x && rect.max_x <= c2.0 {
            // x crosses rect
            return true
        } 
         if c2.0 <= rect.min_x && rect.max_x <= c1.0 {
            // x crosses rect
            return true
        }
        return false
    }
}

fn next_point (point: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    return (point.0 + dir.0, point.1 + dir.1);
}

fn within_extrema (point: (i32, i32), extrema: &Extrema) -> bool {
    return point.0 >= extrema.min_x && point.0 <= extrema.max_x && point.1 >= extrema.min_y && point.1 <= extrema.max_y
}

fn calc_edge (vertex_0: (i32, i32), vertex_1: (i32, i32)) -> Vec<(i32, i32)> {
    let mut edge = vec![];

    if vertex_0.0 != vertex_1.0 {
        if vertex_0.0 < vertex_1.0 {
            for i in vertex_0.0..(vertex_1.0 + 1) {
                edge.push((i, vertex_0.1));
            }
        } else {
            for i in (vertex_1.0..(vertex_0.0 + 1)).rev() {
                edge.push((i, vertex_0.1));
            }
        }
    } else if vertex_0.1 != vertex_1.1 {
        if vertex_0.1 < vertex_1.1 {
            for i in vertex_0.1..(vertex_1.1 + 1) {
                edge.push((vertex_0.0, i));
            }
        } else {
            for i in (vertex_1.1..(vertex_0.1 + 1)).rev() {
                edge.push((vertex_0.0, i));
            }
        }
    }
    return edge;
}

fn parse_input () -> Vec<(i32, i32)> {
    let mut red_riles = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let coords: Vec<i32> = line.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
        red_riles.push((
            coords[0],
            coords[1],
        ));
    }

    return red_riles;
}

#[derive(Debug)]
struct Extrema {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32
}

impl Extrema {
    fn new () -> Extrema {
        return Extrema { min_x: i32::MAX, min_y: i32::MAX, max_x: 0, max_y: 0 }
    }

    fn from_points (corner_1: (i32, i32), corner_2: (i32, i32)) -> Extrema {
        return Extrema {
            min_x: min(corner_1.0, corner_2.0),
            min_y: min(corner_1.1, corner_2.1),
            max_x: max(corner_1.0, corner_2.0),
            max_y: max(corner_1.1, corner_2.1),
        };
    }
}