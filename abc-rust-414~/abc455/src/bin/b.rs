use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        h: usize,
        w: usize,
        a: [String; h],
    }

    let mut grid = vec![Vec::new(); h];
    for (idx, str) in a.iter().enumerate(){
        for j in str.chars(){
            grid[idx].push(j);
        }
    }

    // println!("{:?}", grid);
    let mut cnt = 0;

    for h1 in 0..h{
        for h2 in h1..h{
            for w1 in 0..w{
                for w2 in w1..w{
                    let diff_h = h2 - h1 + 1;
                    let diff_w = w2 - w1 + 1;
                    let mut flag = true;

                    'ret_point: for i in 0..=diff_h / 2{
                        for j in 0..=diff_w / 2{
                            if grid[h1 + i][w1 + j] != grid[h2 - i][w2 - j]{
                                flag = false;
                                break 'ret_point;
                            }
                        }
                    }

                    if flag{
                        // println!("{}, {}, {}, {}",h1, h2, w1, w2);
                        cnt += 1;
                    }
                }
            }
        }
    }
    println!("{}",cnt);
}
