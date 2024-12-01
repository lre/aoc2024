use crate::{Solution, SolutionPair};
use std::{borrow::Borrow, cmp::Ordering, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

fn read_the_numbers() -> (Vec<u32>, Vec<u32>) {
    let inp = read_to_string("inputs/day1-1.txt").unwrap();

    let split: Vec<u32> = inp.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect(); 

    let v1: Vec<u32> = split[0..].iter().step_by(2).copied().collect();
    let v2: Vec<u32> = split[1..].iter().step_by(2).copied().collect();

    (v1, v2)
}   

fn add_and_dum(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    a.iter().zip(b).map(|(a,b)| a.abs_diff(b.clone())).sum()
}

pub fn solve() -> SolutionPair {
    // Your solution here...

    let (mut v1, mut v2) = read_the_numbers();

    v1.sort_unstable();
    v2.sort_unstable();

    let sol1: u32 = add_and_dum(&v1, &v2);

    let mut sol2: u32 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < v1.len() && j < v2.len() {
        match v2[j].cmp(&v1[i]) {
            Ordering::Less => j+=1,
            Ordering::Equal => {
                sol2 += v1[i];
                j += 1;
            }
            Ordering::Greater=> {i += 1}
        }
    }

    (Solution::from(0), Solution::from(0))
}
