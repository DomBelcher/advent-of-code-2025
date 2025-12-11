use std::fs;
use std::collections::{HashMap, HashSet};

// const 
const FILENAME: &str = "./input.txt";
const START_STRING: &str = "you";
const SERVER_STRING: &str = "svr";
const END_STRING: &str = "out";

const FFT_STRING: &str = "fft";
const DAC_STRING: &str = "dac";


fn main() {
    let devices = parse_input();
    // println!("{:?}", devices);

    let graph = build_graph(&devices);
    // let n_paths = find_paths(&graph, &SERVER_STRING.to_string(), &END_STRING.to_string());

    let svr_to_fft = find_paths(&graph, &SERVER_STRING.to_string(), &FFT_STRING.to_string());
    println!("svr to fft: {}", svr_to_fft);

    let fft_to_dac = find_paths(&graph, &FFT_STRING.to_string(), &DAC_STRING.to_string());
    println!("fft to dac: {}", fft_to_dac);
    let dac_to_out = find_paths(&graph, &DAC_STRING.to_string(), &END_STRING.to_string());
    println!("dac to out: {}", dac_to_out);

    println!("total: {}", svr_to_fft as i64 * fft_to_dac as i64 * dac_to_out as i64);
    // println!("There are {} total paths", n_paths);
}

fn find_paths (graph: &HashMap<String, Device>, start_node: &String, end_node: &String) -> i32 {
    let visited_nodes: HashSet<&String> = HashSet::new();
    let mut memo = HashMap::new();

    return do_find(graph, start_node, end_node, &visited_nodes, &mut memo);
}

fn do_find (graph: &HashMap<String, Device>, start_node: &String, end_node: &String, visited_nodes: &HashSet<&String>, memo: &mut HashMap<(String, String), i32>) -> i32 {
    let mut n_paths = 0;
    if start_node == end_node {
        // println!("reached end");
        return 1;
    }
    let mut new_visited_nodes = visited_nodes.clone();
    new_visited_nodes.insert(start_node);
    // println!("visiting node [{}]", start_node);
    // println!("already visited: {:?}", visited_nodes);

    if !graph.contains_key(start_node) {
        // println!("dead end");
        return 0;
        // println!("No node with name {}", start_node);
    }

    let start_device = graph.get(start_node).unwrap();
    for connection in start_device.connected_devices.iter() {
        if !new_visited_nodes.contains(connection) {
            let next_n_paths;
            let key = (connection.clone(), end_node.clone());
            if memo.contains_key(&key) {
                next_n_paths = *memo.get(&key).unwrap()
            } else {
                next_n_paths = do_find(graph, connection, end_node, &new_visited_nodes, memo);
            }
            // let 
            n_paths += next_n_paths;
            memo.insert(key, next_n_paths);
        }
    }

    return n_paths;
}

fn parse_input () -> Vec<Device> {
    let mut input = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        input.push(Device::from_str(line))
    }

    return input;
}

fn build_graph (devices: &Vec<Device>) -> HashMap<String, Device> {
    let mut graph = HashMap::new();

    for device in devices {
        graph.insert(device.name.clone(), device.clone());
    }

    return graph;
}

#[derive(Debug)]
#[derive(Clone)]
struct Device {
    name: String,
    connected_devices: HashSet<String>
}

impl Device {
    fn from_str (input: &str) -> Device {
        let sections = input.split(": ").collect::<Vec<&str>>();
        let connections = sections[1].split_whitespace().collect::<Vec<&str>>();

        let mut connected_devices = HashSet::new();
        for c in connections {
            connected_devices.insert(c.to_string());
        }

        return Device {
            name: sections[0].to_string(),
            connected_devices: connected_devices }
    }
}