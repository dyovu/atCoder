use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: isize,
    }

    let mut ans:isize = 0;

    for i in 1..n+1{
        let s = i*i*i;
        if i%2 == 0{
            ans += s;
        }else{
            ans -= s;
        }
    }

    println!("{}", ans);

}