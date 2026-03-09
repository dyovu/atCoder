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

    let mx:usize = a[..k].iter().sum();

    // println!("mx: {}", mx);

    if mx < x {
        println!("{}", -1);
        return
    }

    let mut sum:usize = 0;
    for (index, &value) in a.iter().enumerate(){
        sum += value;
        if x <= sum {
            println!("{}", index + 1 + n - k);
            return
        }
    }





}