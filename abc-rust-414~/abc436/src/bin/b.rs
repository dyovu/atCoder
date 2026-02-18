use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
    }
    let width = n as isize;
    let mut grid: Vec<Vec<isize>> = vec![vec![width; n]; n];
    grid[0][(n-1)/2] = 1;
    // println!("{:?}", grid);

    let mut cordinate:Vec<isize> = vec![0, (width-1)/2];
    let mut k:isize = 1;

    for _ in 0..n*n-1{
        let mut next_cord_x = ((cordinate[0]-1)%width+width)%width;
        let mut next_cord_y = ((cordinate[1]+1)%width+width)%width;

        // println!("cordinate: {:?}, {}, {}", cordinate, next_cord_x, next_cord_y);
        k += 1;

        if grid[next_cord_x as usize][next_cord_y as usize] != width{
            next_cord_x = ((cordinate[0]+1)%width+width)%width;
            next_cord_y = cordinate[1];
        }

        grid[next_cord_x as usize][next_cord_y as usize] = k;
        cordinate[0] = next_cord_x;
        cordinate[1] = next_cord_y;
    }

    // println!("{:?}", grid);

    for i in grid{
        println!("{}", i.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }

}