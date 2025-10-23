use proconio::input;
use std::process::*;
use std::collections::*;

fn main(){
    input!{
        n : i32,
        a : [i32; n],
    }

    let num_set:BTreeSet<i32>  = a.into_iter().collect();
    
    println!("{}", num_set.len());
    for i in num_set{
      print!("{} ",i);
    }
}