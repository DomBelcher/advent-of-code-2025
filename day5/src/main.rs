use std::{collections::HashSet, fs, cmp::{min, max}};

const FILENAME: &str = "./input.txt";

fn main() {
    let (ranges, ingredients) = parse_input();
    let mut fresh_count = 0;
    let mut range_set = RangeSet::new();

    for range in ranges.iter() {
        range_set.insert(*range);
    }

    for ingredient in ingredients {
        if range_set.contains(ingredient) {
            fresh_count += 1;
        }
    }

    println!("There are {} fresh ingredients", fresh_count);
    println!("Total ranges: {} | condensed ranges: {}", ranges.len(), range_set.len());
    println!("There are {} possible fresh ingredients", range_set.size());
}

fn parse_input () -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut ranges = vec![];
    let mut ingredients = vec![];

    let mut input_mode = "ranges";

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if line.len() == 0 {
            input_mode = "ingredients";
            continue;
        }

        if input_mode == "ranges" {
            let range: Vec<i64> = line.split("-").map(|n| n.parse::<i64>().unwrap()).collect();
            ranges.push((
                range[0], range[1]
            ))
        } else if input_mode == "ingredients" {
            ingredients.push(line.parse::<i64>().unwrap());
        }
    }

    return (ranges, ingredients);
}

fn is_within (value: i64, range: (i64, i64)) -> bool {
    return range.0 <= value && value <= range.1;
}

fn intersects (range_1: (i64, i64), range_2: (i64, i64)) -> bool {
    return is_within(range_1.0, range_2) || is_within(range_1.1, range_2) || is_within(range_2.0, range_1) || is_within(range_2.1, range_1)
}

fn overlap (range_1: (i64, i64), range_2: (i64, i64)) -> Option<(i64, i64)> {
    if !intersects(range_1, range_2) {
        return None
    }

    return Some((min(range_1.0, range_2.0), max(range_1.1, range_2.1)))
}

struct RangeSet {
    ranges: HashSet<(i64, i64)>
}

impl RangeSet {
    fn new () -> RangeSet {
        return RangeSet { ranges: HashSet::new() }
    }

    fn contains (&self, value: i64) -> bool {
        for range in self.ranges.iter() {
            if range.0 <= value && value <= range.1 {
                return true
            }
        }
        return false
    }

    fn size (&self) -> i64 {
        let mut size = 0;
        for range in self.ranges.iter() {
            size += range.1 - range.0 + 1;
        }
        return size
    }

    fn insert (&mut self, candidate_range: (i64, i64)) {
        let mut possible_range = candidate_range;
        let mut deletions = vec![];

        for existing_range in self.ranges.iter() {
            let union = overlap(possible_range, *existing_range);
            if union.is_some() {
                deletions.push(existing_range.clone());
                possible_range = union.unwrap();
            }
        }

        for del in deletions {
            self.ranges.remove(&del);
        }
        self.ranges.insert(possible_range);
    }

    fn len (&self) -> usize {
        return self.ranges.len();
    }
}