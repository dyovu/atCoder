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

    let mut vec = vec![0; 101];
    for i in a{
        let change = ((i + 999) / 1000) * 1000 - i;
        // println!("{}", change);
        vec[100] += change / 100;
        vec[10] += (change % 100) / 10;
        vec[1] += change % 10;
    }
    print!("{} {} {}", vec[1], vec[10], vec[100]);
}
