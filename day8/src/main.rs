use std::collections::{HashSet, HashMap};
use std::fs;

const FILENAME: &str = "./input.txt";
const N_CONNECTIONS: usize = 1000;
const N_LARGEST: usize = 3;

fn main() {
    let junction_boxes = parse_input();
    let n_boxes = junction_boxes.len();
    let mut distances = distances(&junction_boxes);

    let mut n_circuits: usize = 0;
    let mut circuit_mapping = HashMap::new();
    let mut circuits = HashMap::new();

    let mut connected_boxes = HashSet::new();

    let mut part_1_total = 0;
    let mut part_2_total = 0;
    let mut complete = false;
    let mut n_connections = 0;

    while !complete {
        n_connections += 1;

        let (coords, _shortest_dist) = find_shortest_distance(&distances);
        distances.remove(&coords);

        connected_boxes.insert(coords.0);
        connected_boxes.insert(coords.1);

        if !circuit_mapping.contains_key(&coords.0) && !circuit_mapping.contains_key(&coords.1) {
            // neither in a circuit, create new one
            circuit_mapping.insert(coords.0, n_circuits);
            circuit_mapping.insert(coords.1, n_circuits);

            let mut new_circuit = vec![];
            new_circuit.push(coords.0);
            new_circuit.push(coords.1);
            circuits.insert(n_circuits, new_circuit);
            // println!("Creating new circuit for boxes {:?} and {:?}", coords.0, coords.1);

            n_circuits += 1;
        } else if circuit_mapping.contains_key(&coords.0) && circuit_mapping.contains_key(&coords.1) {
            let circuit_0_idx = circuit_mapping.get(&coords.0).unwrap().clone();
            let circuit_1_idx = circuit_mapping.get(&coords.1).unwrap().clone();
            if circuit_0_idx != circuit_1_idx {
                // boxes in different circuits
                // join circuits together

                // println!("Joining circuits {} and {}", circuit_0_idx, circuit_1_idx);

                let circuit_0 = circuits.get(&circuit_0_idx).unwrap().clone();
                let circuit_1 = circuits.get_mut(&circuit_1_idx).unwrap();

                // println!("Circuit {} size: {}", circuit_0_idx, circuit_0.len());
                // println!("Circuit {} size: {}", circuit_1_idx, circuit_1.len());

                for junction in circuit_0 {
                    circuit_1.push(junction);
                    circuit_mapping.insert(junction, circuit_1_idx);
                }
                circuits.remove(&circuit_0_idx);
                // println!("New circuit {} size: {}", circuit_1_idx, circuits.get(&circuit_1_idx).unwrap().len());
            }
            // otherwise junction boxes in same circuit already, do nothing
            // println!("Boxes {:?} and {:?} are in the same circuit", coords.0, coords.1);
        } else if circuit_mapping.contains_key(&coords.0) {
            // coord_1 not in a circuit, add to circuit_0
            let circuit_0_idx = circuit_mapping.get(&coords.0).unwrap().clone();
            circuit_mapping.insert(coords.1, circuit_0_idx);
            circuits.get_mut(&circuit_0_idx).unwrap().push(coords.1);
        } else if circuit_mapping.contains_key(&coords.1) {
            // coord_0 not in a circuit, add to circuit_1
            let circuit_1_idx = circuit_mapping.get(&coords.1).unwrap().clone();
            circuit_mapping.insert(coords.0, circuit_1_idx);
            circuits.get_mut(&circuit_1_idx).unwrap().push(coords.0);
        }

        if n_connections == N_CONNECTIONS {
            println!("n circuits: {}", n_circuits);
            println!("{:?}", circuits);

            let mut circuit_sizes: Vec<(usize, usize)> = circuits.iter().map(|(circuit_idx, boxes)| (*circuit_idx, boxes.len())).collect();
            circuit_sizes.sort_by(|(_a_idx, a_size), (_b_idx, b_size)| b_size.cmp(a_size));

            println!("{:?}", circuit_sizes);

            let mut total = 1;
            for i in 0..N_LARGEST {
                total *= circuit_sizes[i].1
            }
            println!("{}", total);

            part_1_total = total;
        }

        if connected_boxes.len() == n_boxes && circuits.len() == 1 {
            complete = true;
            println!("Last two boxes: {:?}, {:?}", coords.0, coords.1);
            // println!("{}", coords.0.0 * coords.1.0);
            part_2_total = coords.0.0 * coords.1.0
        }
    }

    println!("Part 1 total: {}", part_1_total);
    println!("Part 2 total: {}", part_2_total);

    println!("Made {} connections", n_connections);

}

// it would definitely be more efficient to just sort these, but whatever
fn find_shortest_distance (distances: &HashMap<((i64, i64, i64), (i64, i64, i64)), f64>) -> (((i64, i64, i64), (i64, i64, i64)), f64) {
    let random_coords =  distances.iter().next().unwrap();
    let (mut coords, mut shortest_dist) = (*random_coords.0, *random_coords.1);

    for ((c1, c2), d) in distances {
        if *d < shortest_dist {
            coords = (*c1, *c2);
            shortest_dist = *d;
        }
    }

    return (coords, shortest_dist);
}

fn distances (junction_boxes: &HashSet<(i64, i64, i64)>) -> HashMap<((i64, i64, i64), (i64, i64, i64)), f64> {
    let mut distance_map = HashMap::new();

    for j1 in junction_boxes {
        for j2 in junction_boxes {
            if j1 != j2 && !distance_map.contains_key(&(*j2, *j1)) {
                distance_map.insert((*j1, *j2), dist(*j1, *j2));
            }
        }
    }

    return distance_map;
}

// the sqrt is also not strictly necessary since x >= y -> x^2 >= y^2
fn dist (j1: (i64, i64, i64), j2: (i64, i64, i64)) -> f64 {
    let sq = (j1.0 - j2.0).pow(2) + (j1.1 - j2.1).pow(2) + (j1.2 - j2.2).pow(2);
    return (sq as f64).sqrt();
}

fn parse_input () -> HashSet<(i64, i64, i64)> {
    let mut junction_boxes = HashSet::new();

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let coords: Vec<i64> = line.split(',').map(|v| v.parse::<i64>().unwrap()).collect();
        junction_boxes.insert((
            coords[0],
            coords[1],
            coords[2]
        ));
    }

    return junction_boxes;
}