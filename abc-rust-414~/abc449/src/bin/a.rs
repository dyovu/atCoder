use std::f64::consts::PI;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: f64,
    }
    println!("{}", PI * (n / 2.0).powi(2));
}
