// use proconio::input;
use std::io::{self, BufRead};
// use std::collections::*;

// fn print_type<T>(_: T) {
//     println!("{}", std::any::type_name::<T>());
// }

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for _ in 0..t{
        let cnf = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let n = cnf[0];
        let h = cnf[1];

        let mut sup = h;
        let mut inf = h;
        let mut pre_t:usize = 0;
        

        let mut pass = true;

        for _ in 0..n{
            let query = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let next_t = query[0];
            let duration = next_t - pre_t;
            let l = query[1];
            let u = query[2];

            sup += duration;
            
            if inf < duration{
                inf = 1
            }else{
                inf -= duration;
            }

            // println!("duration={}, sup={}, inf={}, l={}, r={}", duration, sup, inf, l, u);

            if u < inf || sup < l{
                pass = false;
                // break
            }

            sup = sup.min(u);
            inf = inf.max(l);
            pre_t = next_t;
        }

        if pass{
            println!("Yes")
        }else{
            println!("No")
        }

    }



}