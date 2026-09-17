use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        m: usize,
        d: usize,
        s: String
    }

    let mut vec = vec![false; m];

    for (idx, i) in s.chars().enumerate(){
        if i == 'G'{
            let s = ((idx - d) as isize).max(0) as usize;
            for j in s..=(idx + d).min(m - 1){
                vec[j] = true
            }
        }
    }
    // println!("{:?}", vec);
    let cnt = vec.iter().filter(|&&x| !x).count();
    println!("{}", cnt);
}
