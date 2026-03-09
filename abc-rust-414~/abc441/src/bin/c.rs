use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        k: usize,
        x: usize,
        mut a: [usize; n],
    }
    
    a.sort();
    // println!("{:?}", a);

    let mx_alc:usize = a[..k].iter().sum::<usize>();

    // println!("mx: {}", mx);

    if mx_alc < x {
        println!("{}", -1);
        return
    }

    let mut sum:usize = 0;
    for i in 1..=k{
        sum += a[k-i];
        if x <= sum {
            println!("{}", i + n - k);
            return
        }
    }
}