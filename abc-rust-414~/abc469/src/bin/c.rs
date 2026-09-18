use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        s: String,
    }

    let mut iter = s.chars();
    let mut imos:Vec<usize> = (1..=n).collect();
    let mut idx = 0;
    for c in iter{
        if c == 'o'{
            imos[idx] += 1;
        }else {
            idx += 1;
        }
    }
    // println!("{:?}", imos);
    let mut prev = 0;
    for (index, &i) in imos.iter().enumerate(){
        let d = (i + prev - index).min(n);
        println!("{}", d);
        prev = prev.max(d);
    }
}
