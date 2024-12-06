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

    fn leave_trail(self, field: &char) -> char {
        match field {
            '^' => '^',
            '-' => '+',
            '+' => '+',
            '|' => '+',
            '#' => '#',
            '.' => match self {
                Direction::Up => '|',
                Direction::Right => '-',
                Direction::Down => '|',
                Direction::Left => '-',
            },
            _ => panic!("non valid char: {}", field),
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

fn print_matrix(m: &Vec<Vec<char>>, i: usize, j: usize) {
    println!("-------------");
    for (x, v) in m.iter().enumerate(){
        for (y, c) in v.iter().enumerate(){
            if x == i && j == y {
                print!("{}", 'O')
            } else {
                print!("{}", c)
            }
            
        }
        println!("")
    }
    println!("-------------");
}

fn does_loop_exists(mut dir: Direction, m: &Vec<Vec<char>>, mut i: usize, mut j: usize) -> bool {
    let (h, v) = (m.len(), m[0].len());
    (i, j) = dir.update_pos(i, j);

    while i < h && j < v {

        if m[i][j] == '#' {
            (i, j) = dir.turn_right().turn_right().update_pos(i, j);
            match m[i][j] {
                '+' => return true,
                '-' => if '-' == dir.leave_trail(&'.') {return true},
                '|' => if '|' == dir.leave_trail(&'.') {return true},
                _ => {}
                
            }
            dir = dir.turn_right();

        }

        (i, j) = dir.update_pos(i, j);
    }
    false
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let mut sol2: u64 = 0;

    let path = Path::new("inputs/day6-1.txt");
    let mut m: Vec<Vec<char>> = read_file_to_matrix(path);
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let (h, v) = (m.len(), m[0].len());
    let (mut i, mut j) = find_guard(&m, &h, &v).expect("No guard found");
    let mut dir = Direction::Up;

    let mut tracks: Vec<Vec<Option<Direction>>> = vec![vec![None; v]; h];

    while i < h && j < v {
        
        m[i][j] = dir.leave_trail(&m[i][j]);

        if m[i][j] == '#' {
            (i, j) = dir.turn_right().turn_right().update_pos(i, j);
            dir = dir.turn_right();
        } 
            if does_loop_exists(dir.turn_right(), &m, i.clone(), j.clone()) {
                sol2 += 1;
                // println!("Found at {}, {}", i, j);
                let (x, y) = dir.update_pos(i, j);
                //print_matrix(&m , x, y);
            
        }

        seen.insert((i, j));

        
       
        (i, j) = dir.update_pos(i, j);
    }

    (Solution::from(seen.len()), Solution::from(sol2))
}
