use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn main(){
    input!{
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut sum:usize = 0;
    for i in a{
        sum += i;
    }
    if sum <= m{
        print!("Yes")
    }else{
        print!("No")
    }
}