use proconio::input;
// use std::collections::*;

fn main(){
    input!{
        n : usize,
        q : usize,
        x : [usize; q],
    };

    let mut count:Vec<usize> = vec![0; n];
    let mut box_num:Vec<usize> = vec![0; q];

    for (i, v) in x.iter().enumerate(){
        if *v == 0{
            let index = min_min(&count, n);
            box_num[i] = index;
            count[index-1] += 1;
        }else{
            box_num[i] = *v;
            count[*v-1] += 1;
        }
        // println!("index: {}, box_num: {:?}, count: {:?}", i, box_num, count)
    }
    // print!("{:?}", box_num);
    print_vec(&box_num);
}

fn min_min(vec: &Vec<usize>, n:usize) -> usize{
    let mut min_count = 100;
    let mut index = 0;
    for (i, v) in vec.iter().rev().enumerate(){
        if *v <= min_count{
            // println!("index:{}, v:{}, min_count:{}", index, v, min_count);
            min_count = *v;
            index = n-i;
        }
    }
    return index
}

fn print_vec(vec: &Vec<usize>){
    for i in vec{
        print!("{} ", i);
    }
}

