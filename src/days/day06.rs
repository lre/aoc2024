use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

///////////////////////////////////////////////////////////////////////////////

fn read_file_to_matrix<P: AsRef<Path>>(filename: P) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    let file = File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let row = line.expect("no line").chars().collect();
        matrix.push(row);
    }
    matrix
}

fn find_guard(m: &Vec<Vec<char>>, h: &usize, v: &usize) -> Result<(usize, usize), &'static str> {
    for row in 0..*h {
        for col in 0..*v {
            if m[row][col] == '^' {
                return Ok((row, col));
            }
        }
    }

    Err("Guard not found")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn update_pos(self, i: usize, j: usize) -> (usize, usize) {
        match self {
            Direction::Up => (i - 1, j),
            Direction::Right => (i, j + 1),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
        }
    }
}

//fn print_matrix(mut m: Vec<Vec<char>>, seen: &HashSet<(usize, usize, Direction)>, o: (usize, usize)) {
//    println!("-------------");
//    for g in seen {
//        m[g.0][g.1] = match g.2  {
//            Direction::Up => '^',
//            Direction::Right => '>',
//            Direction::Down => 'v',
//            Direction::Left => '<',
//        }
//    }
//    m[o.0][o.1] = 'O';
//    for (_, v) in m.iter().enumerate(){
//        for (_, c) in v.iter().enumerate(){
//                print!("{}", c)
//        }
//        println!("")
//    }
//    println!("-------------");
//}

fn does_loop_exists(
    dir: &Direction, 
    m: &Vec<Vec<char>>, 
    x: &usize, 
    y: &usize, 
    mut seen_dir: HashSet<(usize, usize, Direction)>, 
    seen_pos: &HashSet<(usize, usize)>) -> bool {

    // check if we are adding element on top of guard and inbounds
    let (h, v) = (m.len(), m[0].len());
    
    let object = dir.update_pos(*x, *y);
    if object.0 >= h || object.1 >= v || seen_pos.contains(&(object.0, object.1))
    { 
        return false
    };
    
    let mut loop_dir = dir.turn_right();
    let (mut i, mut j) = (x.clone(), y.clone());
    

    while i < h && j < v {
        if !seen_dir.insert((i, j, loop_dir)) {
            //print_matrix(m.clone(), &seen_dir, (object.0, object.1));
            return  true;
        }

        (i, j) = loop_dir.update_pos(i, j);
        if (i < h && j < v && m[i][j] == '#') || ( i == object.0 && j == object.1)  {
            (i, j) = loop_dir.turn_right().turn_right().update_pos(i, j);
            loop_dir = loop_dir.turn_right();
        } 

    }

    false
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol2: u64 = 0;

    let path = Path::new("inputs/day6-1.txt");
    let m: Vec<Vec<char>> = read_file_to_matrix(path);
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut seen_dir: HashSet<(usize, usize, Direction)> = HashSet::new();

    let (h, v) = (m.len(), m[0].len());
    let (mut i, mut j) = find_guard(&m, &h, &v).expect("No guard found");
    let mut dir = Direction::Up;

    // let mut tracks: Vec<Vec<Option<Direction>>> = vec![vec![; v]; h];

    while i < h && j < v {
               
        seen.insert((i, j));
        seen_dir.insert((i, j, dir));

        if does_loop_exists(&dir, &m, &i, &j, seen_dir.clone(), &seen) {
            sol2 += 1;
        }

        (i, j) = dir.update_pos(i, j);
        if i < h && j < v && m[i][j] == '#' {
            (i, j) = dir.turn_right().turn_right().update_pos(i, j);
            dir = dir.turn_right();
        } 
        
       
    }

    (Solution::from(seen.len()), Solution::from(sol2))
}
