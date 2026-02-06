use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        k: usize,
        s: String,
    }

    let mut count: HashMap<&str, usize> = HashMap::new();

    let mut mx:usize = 0;
    for i in 0..(n - k + 1){
        let sub_str = &s[i..i+k];
        // println!("{}", sub_str);
        let cnt = count.entry(sub_str).or_insert(0);
        *cnt += 1;
        if mx < *cnt {mx =  *cnt};
    }

    // println!("{:?},,, {}", count, mx);
    let mut vec =Vec::new();
    for (sub_str, cnt) in count{
        if cnt == mx {vec.push(sub_str)}
    }
    vec.sort();

    println!("{}", mx);
    println!("{}", vec.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "))


}