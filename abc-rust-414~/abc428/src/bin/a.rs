use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        s: usize,
        a: usize,
        b: usize,
        x: usize,
    }

    let i = x/(a+b);
    let j = x%(a+b);
    let mut ans = a*s*i;
    if j < a{
        ans += j*s;
    }else{
        ans += a*s;
    }

    println!("{}", ans);

}