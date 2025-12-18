use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        s: String,   
    }

    let mut ans = String::new();
    let s_vec: Vec<char> = s.chars().collect();

    let mut flag = true;
    for i in s_vec{
        if i == '#'{
            ans.push(i);
            flag = true;
        }else{
            if flag{
                ans.push('o');
                flag = false;
            }else{
                ans.push(i);
            }
        }
    }

    print!("{}", ans);

}