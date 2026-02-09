use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        x: usize,
        n: usize,
        w: [usize; n],
        q: usize,
        query: [usize; q],
    }

    let mut equiped_parts: HashSet<usize> = HashSet::new();
    let mut weight = x;

    for i in query{
        let index = i-1;
        if equiped_parts.contains(&index){
            equiped_parts.remove(&index);
            weight -= w[index];
        }else{
            equiped_parts.insert(index);
            weight += w[index];
        }

        println!("{}", weight);
    }

}