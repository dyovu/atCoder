use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [isize; n]
    }

    let mut prev = 0;
    let mut back = 0;
    
    let mut p_idx = 0;
    let mut b_idx = n - 1;
    while p_idx <= b_idx{
        if prev < back{
            prev += a[p_idx];
            p_idx += 1;
        }else{
            back += a[b_idx];
            b_idx -= 1;
        }
    }
    // println!("{}, {}", back, prev);
    println!("{}", (back - prev).abs());
}
