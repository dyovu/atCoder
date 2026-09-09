use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        mut a: [[usize; 2]; n],
    }
    
    a.sort_by(|a, b| a[0].cmp(&b[0]));
    let x: Vec<usize> = a.iter().map(|x| x[0]).collect();
    let y: Vec<usize> = a.iter().map(|x| x[1]).collect();
    let mut cnt = 1;
    let mut low_lim = y[0];
    for idx in 1..n{
        if y[idx] < low_lim{
            cnt += 1;
            low_lim = y[idx];
        }
    }

    println!("{}", cnt);
}
