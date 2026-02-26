use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: f64,
    }

    let sqrt_n = n.sqrt();

    // println!("{}", sqrt_n);

    let int_sqrt_n = sqrt_n as usize;
    // println!("{}", int_sqrt_n);

    let mut map:HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    
    for y in 2..=int_sqrt_n{
        for x in 1..y{
            let hype = x.pow(2) + y.pow(2);
            if (n as usize) < hype {continue}
            let entry = map.entry(hype).or_insert(Vec::new());
            entry.push(vec![x, y]);
        }
    }

    let mut ans = Vec::new();
    for (hype, pair )in map.iter(){
        if 1 == pair.len(){
            ans.push(hype);
        }
    }

    println!("{}", ans.len());
    for i in ans{
        print!("{} ", i);
    }


}