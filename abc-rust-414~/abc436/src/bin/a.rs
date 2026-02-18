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

    let l = n - s.len();
    let mut ans = "o".repeat(l);
    ans.push_str(&s);
    println!("{}", ans);

}