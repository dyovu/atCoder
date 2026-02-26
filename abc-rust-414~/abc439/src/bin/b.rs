use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: u32,
    }

    let mut set:HashSet<u32> = HashSet::new();

    let mut ans = n;
    loop {
        ans = ans.to_string().chars().map(|x| x.to_digit(10).unwrap().pow(2)).sum();
        if ans == 1{
            println!("Yes");
            return
        }
        if set.contains(&ans){
            break
        }
        set.insert(ans);
    }

    println!("No")

}