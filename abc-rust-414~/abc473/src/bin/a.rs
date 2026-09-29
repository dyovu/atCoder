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

    let mut sum = 0;
    for i in n / 2..n{
        sum += a[i];
    }
    println!("{}", sum);
}
