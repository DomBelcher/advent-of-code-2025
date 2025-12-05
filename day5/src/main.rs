use std::{collections::HashSet, fs};

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

fn is_within (index: i64, range: (i64, i64)) -> bool {
    return index >= range.0 && index <= range.1;
}

fn is_subrange(range_1: (i64, i64), range_2: (i64, i64)) -> bool {
    return is_within(range_1.0, range_2) && is_within(range_1.1, range_2);
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
        let mut deletions = vec![];
        let mut insertions = vec![];
        let mut is_unique= true;

        for existing_range in self.ranges.iter() {
            if is_subrange(candidate_range, *existing_range){
                is_unique = false;
                break
            }

            if is_subrange(*existing_range, candidate_range) {
                deletions.push(existing_range.clone());
                continue;
            }

            if is_within(candidate_range.0, *existing_range) {
                let mut found_end = false;
                let possible_range = (existing_range.0, candidate_range.1);
                is_unique = false;

                for existing_range_1 in self.ranges.iter() {
                    if is_within(candidate_range.1, *existing_range_1) {

                        deletions.push(existing_range.clone());
                        deletions.push(existing_range_1.clone());
                        insertions.push((existing_range.0, existing_range_1.1));

                        found_end = true;
                    } else if is_subrange(*existing_range_1, possible_range) {
                        deletions.push(existing_range_1.clone());
                    }
                }

                if !found_end {
                    deletions.push(existing_range.clone());
                    insertions.push(possible_range);
                }
                continue
            }

            if is_within(candidate_range.1, *existing_range) {
                let mut found_end = false;
                let possible_range = (candidate_range.0, existing_range.1);
                is_unique = false;

                for existing_range_1 in self.ranges.iter() {
                    if is_within(candidate_range.0, *existing_range_1) {

                        deletions.push(existing_range.clone());
                        deletions.push(existing_range_1.clone());
                        insertions.push((existing_range_1.0, existing_range.1));

                        found_end = true;
                    } else if is_subrange(*existing_range_1, possible_range) {
                        deletions.push(existing_range_1.clone());
                    }
                }

                if !found_end {
                    deletions.push(existing_range.clone());
                    insertions.push(possible_range);
                }
            }
        }

        if is_unique {
            insertions.push(candidate_range);
        }

        for del in deletions {
            self.ranges.remove(&del);
        }
        for ins in insertions {
            self.ranges.insert(ins);
        }
    }

    fn len (&self) -> usize {
        return self.ranges.len();
    }
}

fn condense_ranges (ranges: &Vec<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut condensed_ranges: HashSet<(i64, i64)> = HashSet::new();

    for candidate_range in ranges {
        let mut is_unique = true;
        let mut deletions = vec![];
        let mut insertions = vec![];

        for c_range in condensed_ranges.iter() {
            if is_subrange(*candidate_range, *c_range) {
                // candidate range is within condensed range
                // ignore candidate
                is_unique = false;
                println!("Candidate range {:?} is wholly within range {:?}", candidate_range, c_range);
                if insertions.len() > 0 || deletions.len() > 0 {
                    panic!("oh no");
                }
                break
            }

            if is_subrange(*c_range, *candidate_range) {
                // condensed range is within candidate range
                // delete condensed range
                // insert candidate range
                deletions.push(c_range.clone());
                println!("Candidate range {:?} wholly contains range {:?}", candidate_range, c_range);
                continue;
            }

            if is_within(candidate_range.0, *c_range) {
                is_unique = false;
                let mut found_end = false;

                for c_range_1 in condensed_ranges.iter() {
                    if is_within(candidate_range.1, *c_range_1) {
                        println!("Range {:?} overlaps with ranges {:?} and {:?}", candidate_range, c_range, c_range_1);

                        deletions.push(c_range.clone());
                        deletions.push(c_range_1.clone());
                        insertions.push((c_range.0, c_range_1.1));

                        found_end = true;
                    } else if is_subrange(*c_range_1, (c_range.0, candidate_range.1 )) {
                        deletions.push(c_range_1.clone());
                    }
                }

                if !found_end {
                    deletions.push(c_range.clone());
                    insertions.push((c_range.0, candidate_range.1));
                }
                continue
            }

            if is_within(candidate_range.1, *c_range) {
                is_unique = false;
                let mut found_end = false;

                for c_range_1 in condensed_ranges.iter() {
                    if is_within(candidate_range.0, *c_range_1) {
                        println!("Range {:?} overlaps with ranges {:?} and {:?}", candidate_range, c_range_1, c_range);

                        deletions.push(c_range.clone());
                        deletions.push(c_range_1.clone());
                        insertions.push((c_range_1.0, c_range.1));

                        found_end = true;
                    } else if is_subrange(*c_range_1, (candidate_range.0, c_range.1)) {
                        deletions.push(c_range_1.clone());
                    }
                }

                if !found_end {
                    deletions.push(c_range.clone());
                    insertions.push((candidate_range.0, c_range.1));
                }
                continue
            }
        }

        if is_unique {
            condensed_ranges.insert(*candidate_range);
        }

        for del in deletions {
            condensed_ranges.remove(&del);
        }
        for ins in insertions {
            condensed_ranges.insert(ins);
        }

    }
    return condensed_ranges;
}