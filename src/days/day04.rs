use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

///////////////////////////////////////////////////////////////////////////////

fn read_file_to_matrix<P: AsRef<Path>>(filename: P) -> Vec<Vec<char>>  {
    let mut matrix = Vec::new();
    let file = File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    
    for line in reader.lines() {    
        let row = line.expect("no line").chars().collect();
        matrix.push(row);
    }
    matrix
}

fn check_xmas(m: &Vec<Vec<char>>, i: usize, j: usize, dir: (i32, i32)) -> u64 {
    let mut k = 0;
    let mut i_pos = i.clone() as i32;
    let mut j_pos = j.clone() as i32;

    let xmas = vec!['X', 'M', 'A', 'S'];

    while k < 4 && i_pos >= 0 && j_pos >= 0 && i_pos < m.len() as i32 && j_pos < m[0].len()  as i32{
        let cur_char = m[i_pos as usize ][j_pos as usize];
        
        if cur_char != xmas[k]{
            break;
        }

        i_pos += dir.0;
        j_pos += dir.1;
        k += 1;
    }
    if k == 4 {
        return 1;
    }
    0
}

fn check_xmas_in_all_dir(m: &Vec<Vec<char>>, i: usize, j: usize) -> u64 {
    let directions: Vec<(i32, i32)> = vec![(1,0), (-1, 0), (1, 1), (-1, 1), (0, 1), (0, -1), (1, -1), (-1, -1)];
    let mut found = 0;
    for dir in directions{
        found += check_xmas(m, i, j, dir);
    }
    found
}

fn check_x_mas(m: &Vec<Vec<char>>, i: usize, j: usize, dir1: (i32, i32), dir2: (i32, i32)) -> [char; 2] {
    let mut found = ['X'; 2];

    let mut i_pos = i.clone() as i32 + dir1.0;
    let mut j_pos = j.clone() as i32 + dir1.1;

    if i_pos >= 0 && j_pos >= 0 && i_pos < m.len() as i32 && j_pos < m[0].len()  as i32 {
        found[0] = m[i_pos as usize][j_pos as usize];
    }

    i_pos = i.clone() as i32 + dir2.0;
    j_pos = j.clone() as i32 + dir2.1;

    if i_pos >= 0 && j_pos >= 0 && i_pos < m.len() as i32 && j_pos < m[0].len()  as i32 {
        found[1] = m[i_pos as usize][j_pos as usize];
    }

    found
}

fn check_x_mas_in_all_dir(m: &Vec<Vec<char>>, i: usize, j: usize) -> u64 {
    let dir1: Vec<(i32, i32)> = vec![(1, 1), (-1, -1)];
    let dir2: Vec<(i32, i32)> = vec![(1, -1), (-1, 1)];
    
    let x1 = check_x_mas(m, i, j, dir1[0], dir1[1]);
    let x2 = check_x_mas(m, i, j, dir2[0], dir2[1]);

    if x1.contains(&'M') && x1.contains(&'S') && x2.contains(&'M') && x2.contains(&'S'){
        return  1;
    }

    0
    
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    
    let path = Path::new("inputs/day4-1.txt");
    let m = read_file_to_matrix(path);

    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] == 'X' {
                sol1 += check_xmas_in_all_dir(&m, i, j);
            }
        }
    }

    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] == 'A' {
                sol2 += check_x_mas_in_all_dir(&m, i, j);
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
