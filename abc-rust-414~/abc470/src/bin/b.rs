use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [usize; n],
    }


    let mut vec = vec![0; n + 1];
    for i in a.iter(){
        vec[*i] += 1;
    }
    vec.sort();
    println!("{}", n - vec[n]);
}
