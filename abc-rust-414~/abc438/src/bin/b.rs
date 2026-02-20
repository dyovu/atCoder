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
        s: String,
        t: String,
    }

    let mut ans:isize =  1 << 30;
    for i in 0..=n-m{
        let mut cnt = 0;
        // println!("i: {}",i);
        for j in 0..m{
            // println!("{}, {}", s.chars().nth(j+i).unwrap(), s.chars().nth(j).unwrap());
            let char_s = s.chars().nth(j+i).unwrap().to_digit(10).unwrap() as isize;
            let char_t = t.chars().nth(j).unwrap().to_digit(10).unwrap() as isize;
            cnt +=  ((10 + char_s)-char_t)%10 ;
        }
        // println!("i: {}, cnt: {}",i, cnt);
        ans = ans.min(cnt);
    }

    println!("{}", ans);

}