use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

use num_bigint::BigInt;
use num_traits::Zero;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        mut a: [usize; n],
    }

    let len = a.len();
    a.sort();
    a.reverse();

    let last = a.last().unwrap();

    let mut prev_index = 0;
    let mut next = a.first().unwrap();

    let mut ans: BigInt = BigInt::from(0);

    for i in 0..n {
        let next = if i + 1 < n { a[i + 1] } else { 0 };
        let d = a[i] - next;
        for _ in 0..d {
            ans = &ans * 10u32 + (i + 1) as u32;
        }
    }

    println!("{}", ans);

}