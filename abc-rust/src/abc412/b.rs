use proconio::input;
// use std::collections::*;

fn main(){
    input!{
        n : String,
        t : String
    };

    let mut is_upper = false;

    for i in n.chars().rev(){
        // println!("i: {}, {}", i, is_upper);
        if is_upper{
            if !t.contains(i){
                println!("No");
                std::process::exit(0)
            }
            is_upper=false;
        }
        if i.is_uppercase(){
            is_upper = true;
        }
    }

    println!("Yes");

}