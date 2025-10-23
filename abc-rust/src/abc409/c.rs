// use proconio::input;
// // use std::collections::*;

// fn main(){
//     input!{
//         n : usize,
//         l : usize,
//         a : [usize; n-1],
//     }

//     // 円周が3の倍数出ない時は、絶対に正三角形が作れない。（入力が整数のため）
//     if l%3 != 0{
//         print!("{}", 0);
//         std::process::exit(0);
//     }
//     let distance = l/3;
//     // println!("{}", distance);

//     let mut count = vec![0; l];

//     count[0] = 1;

//     let mut current_position = 0;
//     for i in a{
//         current_position = (current_position+i)%l;
//         count[current_position] += 1;
//     }

//     // println!("{:?}", position_map);

//     let mut ans:usize = 0;

//     for i in 0..distance{
//         let c1 = count[i];
//         let c2 = count[i + distance];
//         let c3 = count[i + distance*2];

//         ans += c1 * c2 * c3;
//         // println!("i : {} {}", i ,ans);
//     }

//     println!("{}", ans);



// }


use proconio::input;
use std::collections::*;

fn main(){
    input!{
        n : usize,
        l : usize,
        a : [usize; n-1],
    }

    // 円周が3の倍数出ない時は、絶対に正三角形が作れない。（入力が整数のため）
    if l%3 != 0{
        print!("{}", 0);
        std::process::exit(0);
    }
    let distance = l/3;
    // println!("{}", distance);

    let mut position_map = BTreeMap::new();
    for i in 0..l{
        position_map.insert(i, 0 as usize);
    }
    position_map.insert(0, 1);

    let mut current_position = 0;
    for i in a{
        current_position = (current_position+i)%l;
        let count = position_map.entry(current_position).or_insert(0);
        *count += 1;
    }

    // println!("{:?}", position_map);

    let mut ans = 0;

    for i in 0..distance{
        let c1 = *position_map.get(&i).unwrap_or(&0);
        let c2 = *position_map.get(&(i+distance)).unwrap_or(&0);
        let c3 = *position_map.get(&((i+distance*2)%l)).unwrap_or(&0);

        ans += c1 * c2 * c3;
        // println!("i : {} {}", i ,ans);
    }
    println!("{}", ans);
}