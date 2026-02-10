use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        s: String, 
    }

    let mut count:Vec<(u32, u32)> = Vec::new();
    let first = s.chars().nth(0).unwrap().to_digit(10).unwrap();
    count.push((first, 0 as u32));
    // println!("{:?}", count);

    for i in s.chars(){
        if i.to_digit(10).unwrap() == count.last().unwrap().0{
            count.last_mut().unwrap().1 += 1;
        }else{
            count.push((i.to_digit(10).unwrap(), 1))
        }
    }

    // println!("{:?}", count);

    let mut ans = 0;
    for i in 0..count.len()-1{
        let first_half = count[i].0;
        let second_half = count[i+1].0;
        if first_half + 1 != second_half{
            continue
        }

        let first_count = count[i].1;
        let second_count = count[i+1].1;
        ans += first_count.min(second_count);
    }

    println!("{}", ans)

}