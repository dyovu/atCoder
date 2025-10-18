use proconio::input;
use std::collections::*;

fn main(){
    input!{
        n : i32,
        a : [i32; n],
    }

    let num_set:HashSet<i32>  = a.into_iter().collect();
    let result_vec = num_set.into_iter().collect();

    result_vec.sort();
    println!("{:?}", result_vec);
}