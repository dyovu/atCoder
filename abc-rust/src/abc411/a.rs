use proconio::input;
// use std::collections::*;

fn main(){
    input!{
        p : String,
        l : usize,
    };

    if p.len() < l{
        print!("No");
    }else{
        print!("Yes");
    }


}