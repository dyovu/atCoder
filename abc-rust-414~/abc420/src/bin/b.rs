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
        a: [String; n],
    }

    let mut score:Vec<usize> = vec![0; n];

    for i in 0..m{
        let mut x:Vec<usize> = Vec::new();
        let mut y:Vec<usize> = Vec::new();

        for j in 0..n{
            let vote = a.get(j).unwrap().chars().nth(i).unwrap();
            if vote == '0'{
                x.push(j);
            }else{
                y.push(j);
            }
        }

        // println!("x: {:?}, y: {:?}", x, y);
        if x.len() == 0{
            for j in y{
                score[j] += 1;
            }
            continue
        }else if y.len()==0{
            for j in x{
                score[j] += 1;
            }
            continue
        }

        if x.len() > y.len(){
            for j in y{
                score[j] += 1;
            }
        }else{
            for j in x{
                score[j] += 1;
            }
        }
    }

    // println!("{:?}", score);
    // println!("{:?}", score);

    let max = score.iter().max().unwrap();

    for (index, value) in score.iter().enumerate(){
        if value == max {
            print!("{} ", index+1);
        }
    }

}