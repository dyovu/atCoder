use proconio::input;
// use std::collections::*;

fn main(){
    input!{
        n : usize,
        d : [usize; n-1],
    };

    for index in 0..n-1{
        let mut distance: usize = 0;
        for i in index..n-1{
            // println!("index: {}, i: {}", index, i);
            distance += d[i];
            print!("{} ", distance)
        }
        println!();
    }
}