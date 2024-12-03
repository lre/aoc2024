use regex::Regex;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    let s = read_to_string("inputs/day3-1.txt").expect("No file found");
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut mul = true;
    for cap in re.captures_iter(&s) {
        match &cap[0] {
            "do()" => mul = true,
            "don't()" => mul = false,
            _ => {
                let x = &cap[1].parse::<u64>().expect("not a number ");
                let y = &cap[2].parse::<u64>().expect("not a number ");
        
                sol1 += x * y;
                
                if mul {
                    sol2 += x * y;
                }
            }
        }

    }

    (Solution::from(sol1), Solution::from(sol2))
}
