use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [(usize, usize, String); n],
    }

    let mut cnt = 0;
    for (price, paid, flag) in a{
        if flag.as_str() == "keep"{
            cnt += paid - price;
        }
    }
    println!("{}", cnt)
}
