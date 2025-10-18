use proconio::input;

fn main(){
    input!{
        n : i32,
        s : i32,
        t : [i32;n],
    }
    let mut current:i32 = 0;

    // println!("{:?}", t);

    for i in t{
        if current + s < i{
            print!("No");
            return;
        }
        current = i;
        // println!("{}", current)
    }
    print!("Yes")
}