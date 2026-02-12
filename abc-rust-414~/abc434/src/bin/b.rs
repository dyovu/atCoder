use proconio::input;
// use std::io::{self, BufRead};
// use std::{collections::*, hash::Hash};

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
        a: [[usize; 2]; n],
    }

    let mut ss_weight:Vec<Vec<usize>> = vec![Vec::new(); m];

    for i in a.iter(){
        let s = i[0];
        let weight = i[1];

        ss_weight[s-1].push(weight);
    }

    for i in ss_weight{
        let ave:f64 = i.iter().sum::<usize>() as f64 /i.len() as f64;
        println!("{}", ave);
    }

}