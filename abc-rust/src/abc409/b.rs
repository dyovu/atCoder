use proconio::input;

fn main(){
    input!{
        n : usize,
        mut a : [i32; n],
    }

    a.sort_by(|x, y| y.cmp(x));

    let mut ans = 0;
    for (k, v) in a.iter().enumerate(){
        if *v >= (k + 1) as i32{
            ans += 1;
        }
    }

    println!("{}", ans);
}