use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
        a: [[usize; 2]; n],
    }

    let mut count: Vec<[isize; 2]> = vec![[0; 2]; m];
    
    for vec in a{
        for i in 0..2{
            count[vec[i] - 1][i] += 1;
        }
    }
    
    for i in count{
        println!("{}", i[1] - i[0]);
    }
}
