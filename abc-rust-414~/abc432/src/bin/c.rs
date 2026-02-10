use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
    }

    let y_minus_x = y-x;
    let r = (a[0]*x)%y_minus_x;

    a.sort();
    let max_T = a[0]*y;
    let min_T = a[n-1]*x;

    if max_T < min_T{
        println!("{}", -1);
        return
    }

    for i in a.iter(){
        if (i*x)%y_minus_x != r{
            println!("{}", -1);
            return
        }
    }


    let T = y_minus_x*((max_T-r)/y_minus_x) + r;
    if T < min_T{
        println!("{}", -1);
        return
    }

    let mut ans = 0;
    for i in a{
        ans += (T-x*i)/(y-x);
    }

    println!("{}", ans);

}

