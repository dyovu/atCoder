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
        mut c: [usize; m],
        a: [[usize; 2]; n],
    }

    let mut cnt = 0;
    for elem in a.iter(){
        let num = elem[0];
        let quant= elem[1];
        if c[num - 1] > 0{
            let con = quant.min(c[num - 1]);
            cnt += con;
            c[num - 1] -= con;
        }
    }
    println!("{}", cnt);
}
