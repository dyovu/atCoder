use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: String,
    }
        
    let mut iter = n.chars();
    let mut cnt: usize = 1;
    let mut ans: usize = 0;
    let mut prev: char = iter.next().unwrap();

    for i in iter{
        if i != prev{
//            println!("i: {}, cnt: {}",i,  cnt);
            cnt += 1;
            prev = i;
            continue;
        }

 //       println!("{}", cnt);
        ans += cnt * (cnt + 1) / 2;
        cnt = 1;
    }

    ans += cnt * (cnt + 1) / 2;
    println!("{}", ans % 998244353);
}
