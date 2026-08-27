use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        s: String,
        t: String,
    }

    let s_removed_a = s.chars().filter(|x| *x != 'A').collect::<String>();
    let t_removed_a = t.chars().filter(|x| *x != 'A').collect::<String>();

    if s_removed_a != t_removed_a{
        println!("-1");
    }else {
        let mut cnt = 0;
        let mut s_iter = s.chars();
        let mut t_iter = t.chars();
        
        for i in t_removed_a.chars(){
            let mut diff: i32 = 0;

            while s_iter.next() != Some(i){
                diff += 1;
            }
            while t_iter.next() != Some(i){
                diff -= 1;
            }
            // println!("diff: {}", diff);
            cnt += diff.abs();
        }
        let mut diff: i32 = 0;

        while s_iter.next() != None{
            diff += 1;
        }
        while t_iter.next() != None{
           diff -= 1;
        }     
        cnt += diff.abs();
        println!("{}", cnt);
    }
}
