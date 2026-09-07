use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        mut m: usize,
    }

    let mut cnt = 0;
    while 0 < m{
        m = n % m;
        cnt += 1;
    }
    println!("{}", cnt);
}
