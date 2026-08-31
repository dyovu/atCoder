use std::path::Ancestors;

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
    
    let mut ans = 0;

    for i in 0..1 << n{
        let mut cnt = 0;
        let mut pos: isize = 1;
        for (idx, &val) in a.iter().enumerate(){
            if (i >> idx) & 1 == 0 {
                let prev = pos;
                pos -= (val as isize) * 2;
                if prev.signum() * pos.signum() < 0{
                    cnt += 1;
                }
            }else if (i >> idx) & 1 == 1 {
                let prev = pos;
                pos += (val as isize) * 2;
                if prev.signum() * pos.signum() < 0{
                    cnt += 1;
                }
            }
        }
        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
