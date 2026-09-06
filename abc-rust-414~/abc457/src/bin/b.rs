// use proconio::input;
use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut arr_2d: Vec<Vec<usize>> = Vec::new();
    for i in 0..n{
        let arr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        arr_2d.push(arr);
    }
    let q: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    println!("{}", arr_2d[q[0] - 1][q[1]]);
}
