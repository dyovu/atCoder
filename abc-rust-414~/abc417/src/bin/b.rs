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
        a: [usize; n],
        b: [usize; m],
    }

    let mut sub_vec  = a;
    for i in b{
        if let Ok(index) = sub_vec.binary_search(&i){
            sub_vec.remove(index);
        }
    }

    
    for i in sub_vec{
        print!("{} ", i);
    }

}