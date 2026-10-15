use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        q: usize,
        a: [[usize; 3]; q],
    }

    let mut vec: Vec<isize> = vec![0; n];
    let mut map: HashMap<usize, BTreeSet<(usize, usize)>> = HashMap::new();
    for i in a{
        let entry = map.entry(i[2]).or_insert(BTreeSet::new());
        entry.insert((i[0], i[1]));
    }
    // println!("{:?}", map);

    for (i, set) in map{
        let mut set_iter = set.iter();
        let mut first = set_iter.next().unwrap();
        let mut l = first.0;
        let mut r = first.1;
        for &(l_next, r_next) in set_iter{
            if l_next <= r{
                r = r.max(r_next);
            }else{
                vec[l - 1] += 1;
                if r != n{
                    vec[r] -= 1;
                }
                l = l_next;
                r = r_next;
            }
        }
        vec[l - 1] += 1;
        if r != n{
            vec[r] -= 1;
        }
    }

    // println!("{:?}", vec);
    let mut ans = 0;
    for (i, val) in vec.iter().enumerate(){
        ans += val;
        if i != 0{
            print!(" ");
        }
        print!("{}", ans);
    }
}
