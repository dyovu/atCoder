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

    let mut cum_sum = vec![0; n+1];

    for (index, value) in a.iter().enumerate(){
        cum_sum[index+1] = cum_sum[index] + *value;
    }

    let mut count = 0;

    for l in 0..n{
        for r in l..n{
            let sub_sum = cum_sum[1+r] - cum_sum[l];
            // println!("l, r = {} , {}  sub_sum: {}", l, r, sub_sum);
            let mut is_divisor = false;
            
            for i in l..=r{
                if sub_sum%a[i] == 0{
                    // println!("sub_sum%a[i]: {}" , sub_sum%a[i]);
                    is_divisor = true;
                    break
                }
            }
            if !is_divisor {count +=1};
        }
    }

    println!("{}", count);



}