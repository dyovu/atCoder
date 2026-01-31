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
        k: usize,
        a: [[usize; 2]; k],
    }

    let mut ans_count:Vec<usize> = vec![0; n+1];

    for i in a.iter(){
        let k = i[0];
        ans_count[k] += 1;
        if ans_count[k] == m{

            print!("{} ", k)
        }
    }

}