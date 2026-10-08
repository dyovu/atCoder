use std::cmp;

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

    let mut vec = vec![0; 3];
    vec[0] = a[0];
    vec[1] = a[1];
    vec[2] = a[2];
    vec.sort_by(|a, b| b.cmp(a));
    println!("{}", vec[2]);
    for &i in &a[3..n]{
        if vec[0] < i{
            vec[2] = vec[1];
            vec[1] = vec[0];
            vec[0] = i;
        }else if vec[1] < i{
            vec[2] = vec[1];
            vec[1] = i;
        }else if vec[2] < i{
            vec[2] = i;
        }
        println!("{}", vec[2]);
    }
}
