use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

const four_neighborhood:[[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn main(){
    input!{
        h: isize,
        w: isize,
        a: [String; h],
    }

    for (i, row) in a.iter().enumerate(){
        for (j, value) in row.chars().enumerate(){
            if value == '.' {continue}

            let mut count = 0;
            for k in four_neighborhood{
                let x = j as isize - k[1];
                let y = i as isize - k[0];

                if x < 0 ||  y < 0 || h-1 < y || w-1 < x{
                    continue
                }
                if a[y as usize].chars().nth(x as usize).unwrap() == '#'{
                    count += 1;
                }
            }

            if count != 2 && count != 4{
                println!("No");
                return;
            }
        }
    }

    println!("Yes");

}