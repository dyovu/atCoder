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
        a: [usize; n],
    }

    let mut sum = 0;
    let mut ans = vec![true; n];
    for i in 0..n{
        if m <= i{
            if ans[i - m]{
                sum -= a[i - m];
            }
        }
        if sum + a[i] <= k{
            sum += a[i];
            ans[i] = true;
        }else{
            ans[i] = false;
        }
    }
    for i in ans{
        if i {
            println!("Yes");
        }else {
            println!("No");
        }
    }
}
