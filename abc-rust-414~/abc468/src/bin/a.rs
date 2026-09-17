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
    
    let mut cnt = 0;
    for i in 0..n - 2{
        if a[i] < a[i + 1] && a[i + 2] < a[i + 1]{
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
