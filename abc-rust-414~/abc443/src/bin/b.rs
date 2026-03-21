use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        mut n: usize,
        k: usize,
    }

    let mut age = n;
    let mut cnt = n;

    while cnt < k{
        age += 1;
        cnt += age;   
    }

    println!("{}", age-n);

}