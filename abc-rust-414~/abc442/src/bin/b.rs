use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        q: usize,
        query: [usize; q],
    }

    let mut is_playback = false;
    let mut volume:usize  = 0;

    for i in query.iter(){
        match i {
            1 => {
                volume += 1;
            }
            2 => {
                if volume != 0{
                    volume -=1;
                }
            }
            3 => {
                is_playback = !is_playback;
            }
            _ => {}
        }

        if is_playback && 3 <= volume{
            println!("Yes")
        }else{
            println!("No")
        }
    }

}