use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        h: usize,
        w: usize,
        input: [String; h],
    }

    let mut start :[usize; 2] = [0, 0];
    let mut end :[usize; 2] = [w - 1, h - 1];

    let mut grid: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for (i, str) in input.iter().enumerate(){
        for (j, c) in str.chars().enumerate(){
            let v = if c == '#' {1} else {0};
            grid[i][j] = v;
        }
    }

    while grid[start[1]].iter().filter(|&&x| x == 0).count() == w{
        start[1] += 1;
    }
    while grid[end[1]].iter().filter(|&&x| x == 0).count() == w{
        end[1] -= 1;
    }
    while grid.iter().filter(|&x| x[start[0]] == 0).count() == h{
        start[0] += 1;
    }
    while grid.iter().filter(|&x| x[end[0]] == 0).count() == h{
        end[0] -= 1;
    }
    // println!("{:?}, {:?}", start, end);

    for i in start[1]..=end[1]{
        for j in start[0]..=end[0]{
            let v = if grid[i][j] == 1 {'#'} else {'.'};
            print!("{}", v);
        }
        println!();
    }
}
