use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


///////////////////////////////////////////////////////////////////////////////

fn is_safe_up(nums: &[i32], step: i32) -> bool {
    
    for i in 1..nums.len() {
        let new = nums[i-1] + step;
        if nums[i-1] >= nums[i] || nums[i] > new {
            return false;
        }
    }

    true
}


fn is_safe_up_dampener(nums: &[i32], step: i32) -> bool {
    for i in 1..nums.len() {
        let new = nums[i-1] + step;
        if nums[i-1] >= nums[i] || nums[i] > new {
            // run is_safe_up with nums where index i-1 is removed 
            let mut new_nums = nums.to_vec();
            new_nums.remove(i-1);
            if is_safe_up(&new_nums, step) {
                return true;
            }
            
            // run is_safe_up with nums where index 1 is removed
            let mut new_nums = nums.to_vec();
            new_nums.remove(i);
            if is_safe_up(&new_nums, step) {
                return true;
            }
            
            return false;
        }
    }
    true
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    let path = Path::new("inputs/day2-1.txt");
    let file = File::open(path).expect("Could not open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let o: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().expect("not a number ")).collect();
        let mut o_reversed = o.clone();
        o_reversed.reverse();

        if is_safe_up(&o, 3) || is_safe_up(&o_reversed, 3){
            sol1 += 1;
        }

        if is_safe_up_dampener(&o, 3) || is_safe_up_dampener(&o_reversed, 3){
            sol2 += 1;
        }

    }

    (Solution::from(sol1), Solution::from(sol2))
}
