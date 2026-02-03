use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        q: usize,
    }

    let mut count = vec![0; n + 1];
    for i in 1..=n {
        count[i] = 1; 
    }

    let mut min_v = 1; 

    for _ in 0..q {
        input!{
            x: usize,
            y: usize,
        }
        
        if x < min_v {
            println!("0");
            continue;
        }
        
        let mut upgraded = 0;
        for v in min_v..=x {
            upgraded += count[v];
            count[v] = 0;
        }
        count[y] += upgraded;
        
        println!("{}", upgraded);
        min_v = x + 1;
    }
}