use std::io::BufRead;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let mut lines = std::io::stdin().lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for i in 0..t{
        let situation:Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let n = situation[0];
        let w = situation[1];
        let c:Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();

    }


}