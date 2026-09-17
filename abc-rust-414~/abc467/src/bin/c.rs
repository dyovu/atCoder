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
        a: [usize; n],
        b: [usize; n - 1],
    }

    let mut rem = Vec::new();
    for i in 0..n - 1{
        rem.push((a[i] + a[i + 1]) % 2);
    }
    
    let mut r: usize = 0;
    let mut l = 0;
    let mut tmp_a = a.clone();
    for i in 0..n - 1{
        if (tmp_a[i] + tmp_a[i + 1]) % 2 != b[i]{
            tmp_a[i + 1] += 1;
            r += 1;
        }
    }
    tmp_a = a.clone();
    tmp_a[0] += 1;
    for i in 0..n - 1{
        if (tmp_a[i] + tmp_a[i + 1]) % 2 != b[i]{
            tmp_a[i + 1] += 1;
            l += 1;
        }
    }
    println!("{}", r.min(l));
}
