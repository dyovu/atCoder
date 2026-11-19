use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        w: usize,
        a: [[usize; 2]; n],
    }

    let mut ans = 0;
    for i in 0..(1 << n){
        let mut value = 0;
        let mut weight = 0;
        for idx in 0..n{
            let f = (i >> idx) & 1;
            if f == 1{
                value += a[idx][0];
                weight += a[idx][1];
            }
        }
        // println!("value{}, weight{}", value, weight);
        if weight <= w && ans <= value{
            ans = value;
        }
    }
    println!("{}", ans);
}
