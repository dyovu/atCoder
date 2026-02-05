use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn rec_sum(n:usize) -> usize{
    if n == 0{
        return 1;
    }
    let tmp = rec_sum(n-1);

    let mut sum:usize = tmp +  tmp.to_string().chars().map(|c| c.to_digit(10).unwrap() as usize).sum::<usize>();
    
    // println!("n = {}, sum : {}", n, sum);
    return sum;
}

fn main(){
    input!{
        n: usize,
    }

    let ans = rec_sum(n-1);

    println!("{}", ans);
}

