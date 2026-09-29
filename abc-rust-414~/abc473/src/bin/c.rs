use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut vec: Vec<usize> = vec![0; k];
    for &i in a.iter(){
        vec[i - 1] += 1;
    }
    vec.sort();
    let mx = vec[k - 1];
    let idx = vec.partition_point(|&x| x < mx - 1);
    println!("{}", k - idx);
}
