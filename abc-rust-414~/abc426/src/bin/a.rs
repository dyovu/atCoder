use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}


fn main(){
    input!{
        x: String,
        y: String,
    }
    let version:HashMap<&str, i32> = HashMap::from(
        [("Ocelot", 1), ("Serval", 2), ("Lynx", 3)]
    );

    let x_v = version[x.as_str()];
    let y_v = version[y.as_str()];

    if y_v <= x_v{
        println!("Yes")
    }else{
        println!("No");
    }
}