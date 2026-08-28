use proconio::input;
use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut h: usize = first_line[0];
    let mut w: usize = first_line[1];
    let mut q: usize = first_line[2];

    for i in lines.take(q){
        let query: Vec<usize> = i.unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        match query[0]{
            1 => {
                println!("{}", query[1] * w);
                h -= query[1];
            }
            2 => {
                println!("{}", query[1] * h);
                w -= query[1];
            }
            _ => {}
        }
    }    
}
