use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let mut s = 0;
    for i in 0..n{
        if q[i] < p[i]{
            println!("0");
            std::process::exit(0);
        }else if p[i] < q[i]{
            break;
        }
        s += 1;
    }
    
    let mut ans = 0;
    let mut cnt = 0;
    for i in s + 1..n{
        if p[s] < p[i] && p[i] < q[s]{
            ans += 1;
        }
    }
    ans *= n - s - 1;
    
    for i in s + 1..n - 1{
        for j in i + 1..n{
            if p[i] < p [j]{
                ans += 1;
            }
            if q[j] < q[i]{
                ans += 1;
            }
        }
        ans *= n - i - 1;
    }
    
    println!("{}", ans);
}
