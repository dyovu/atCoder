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
        mut sh: [usize; n],
        mut ne: [usize; m],
    }

    sh.sort();
    ne.sort();
    let mut idx_ne = 0;
    let mut cnt = 0;
    for i in sh{
        if m == idx_ne{
            break;
        }
        if ne[idx_ne] <= i * 2{
            cnt += 1;
            idx_ne += 1;
        }
    }
    println!("{}", cnt);
}
