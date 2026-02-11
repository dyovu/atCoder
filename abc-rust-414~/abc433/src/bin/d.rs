use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
        a: [isize; n],
    }

    let mut rem_digit:Vec<Vec<usize>> = vec![Vec::new(); 11];
    // println!("rem_digit : {:?}", rem_digit);

    for i in a.iter(){
        let digit = i.to_string().len();
        rem_digit[digit].push((((-i)%(m as isize) + m as isize)%(m as isize ))  as usize);
    }

    for i in rem_digit.iter_mut(){
        i.sort();
    }

    // println!("rem_digit : {:?}", rem_digit);

    let mut ans:usize = 0;
    for i in a{
        for (digit, rem) in rem_digit.iter().enumerate(){
            if rem.is_empty(){
                continue;
            }

            

            let rem_i = (10usize.pow(digit as u32)*i as usize)%m;
            let l = rem.partition_point(|x| *x < rem_i);
            let r = rem.partition_point(|x| *x <= rem_i);

            // println!("digit = {}, i={}, rem_i = {}, l = {}, r = {}", digit, i, rem_i, l, r);
            ans += r - l;
        }
    }

    println!("{}", ans);


    



}