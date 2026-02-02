use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [isize; n],
    }

    let mut p:Vec<usize> = vec![0; n];
    let mut set:HashSet<usize> = HashSet::new();

    for (index, value) in a.iter().enumerate(){
        if *value != -1{
            let v = *value as usize;
            if set.contains(&v){
                println!("No");
                return
            }
            set.insert(v);
            p[index] = v;
        }
    }

    let set_n: HashSet<usize> = (1..=n).collect();
    let mut difference: HashSet<_> = set_n.difference(&set).copied().collect();

    // println!("{:?}", difference);

    for i in 0..n{
        if p[i] == 0{
            if let Some(&value) = difference.iter().next() {
                difference.remove(&value);
                p[i] = value;
                // println!("取り出した値: {:?}", value);
            }
        }
    }



    // println!("{:?}", p);
    println!("Yes");
    println!("{}", p.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));

}