use proconio::input;
// use std::collections::*;

fn main(){
    input!{
        n : usize,
        a : [usize; n],
        k : usize,
    };

    let mut count = 0;
    for i in a{
        println!("{}, {}", i, k);
        if i >= k{
            count += 1;
        }
    }

    print!("{}", count);

}