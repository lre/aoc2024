use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

use std::path::Path;

/////////////////////AAAHHHHH THIS IS NOT GOOD CODE !!!!! /////////////////////////////////////

fn read_file_to_thing<P: AsRef<Path>>(filename: P) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let file = File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates = Vec::new();

    let mut read_rules = true;

    for line in reader.lines() {
        let row = line.expect("no line");
        if row.is_empty() {
            read_rules = false;
            continue;
        }

        if read_rules {
            let nums: Vec<&str> = row.split("|").collect();
            let first = nums[0].parse::<u32>().unwrap();
            let second = nums[1].parse::<u32>().unwrap();
            rules.push((first, second));
        } else {
            let nums: Vec<u32> = row
                .split(",")
                .map(|x| x.parse::<u32>().expect("nan"))
                .collect();
            updates.push(nums);
        }
    }
    (rules, updates)
}

fn get_middle_from_sorted(
    rule_lookup: HashMap<u32, Vec<u32>>,
    updates: Vec<u32>,
) -> Result<u32, u32> {
    let mut should_have_been_first: HashMap<u32, u32> = HashMap::new();
    let mut where_am_i: HashMap<u32, usize> = HashMap::new();
    let mut new_update: Vec<u32> = Vec::new();
    let mut has_swap = false;

    for (index, u) in updates.iter().enumerate() {
        if let Some(v) = should_have_been_first.get(u) {
            //println!("where_am_i {:?}", where_am_i);
            let index_2 = where_am_i.get(v).expect("where_am_i").clone();
            where_am_i.insert(v.clone(), index);
            where_am_i.insert(u.clone(), index_2);

            //swap
            let t = new_update[index_2];
            new_update[index_2] = u.clone();
            new_update.push(t);
            has_swap = true;
        } else {
            new_update.push(u.clone());
        }

        if let Some(value) = rule_lookup.get(u) {
            for k in value {
                should_have_been_first.insert(k.clone(), u.clone());
                where_am_i.insert(u.clone(), index);
            }
        };
    }

    //println!("new_update {:?}", new_update);
    if has_swap {
        return get_middle_from_sorted(rule_lookup, new_update);
    }
    return Ok(new_update[new_update.len() / 2].clone());
}

fn get_middle_if_valid(
    rule_lookup: &HashMap<u32, Vec<u32>>,
    updates: &Vec<u32>,
) -> Result<u32, u32> {
    let mut should_have_been_first: HashSet<u32> = HashSet::new();

    for u in updates {
        //// println!("u {:?}", u);
        //// println!("should_have_been_first {:?}", should_have_been_first);
        if should_have_been_first.contains(u) {
            return Err(0);
        }

        if let Some(value) = rule_lookup.get(u) {
            should_have_been_first.extend(value);
        };

        //// println!(" -------- ");
    }

    //// println!("update {:?}", updates);
    Ok(updates[updates.len() / 2])
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u32 = 0;
    let mut sol2: u32 = 0;

    let path = Path::new("inputs/day5-1.txt");
    let (rules, updates) = read_file_to_thing(path);

    let mut rule_lookup: HashMap<u32, Vec<u32>> = HashMap::new();

    for (f, l) in rules {
        rule_lookup.entry(l).or_insert_with(Vec::new).push(f);
    }

    //let mut unsorted_update = Vec::new();

    for update in updates {
        sol1 += match get_middle_if_valid(&rule_lookup, &update) {
            Ok(x) => {
                //// println!("{}", x);
                x
            }
            Err(_) => {
                sol2 += match get_middle_from_sorted(rule_lookup.clone(), update.clone()) {
                    Ok(x) => {
                        //println!("{}", x);
                        x
                    }
                    Err(_) => 0,
                };
                0
            }
        };
    }

    (Solution::from(sol1), Solution::from(sol2))
}
