use std::collections::HashMap;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    for i in a{
        *map.entry(i).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (i, v) in map{
        if v % 2 != 0{
            ans += i;
        }
    }
    println!("{}", ans);
}
