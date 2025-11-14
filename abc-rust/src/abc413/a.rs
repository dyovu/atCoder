use proconio::input;
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