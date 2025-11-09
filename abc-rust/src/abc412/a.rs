use proconio::input;
// use std::collections::*;

fn main(){
    input!{
        n : usize,
        arr : [[usize; 2]; n],
    };

    let mut count:usize = 0;
    for i in 0..n{
        if arr[i][0] < arr[i][1]{
            count += 1;
        }
    }

    println!("{}", count);
}