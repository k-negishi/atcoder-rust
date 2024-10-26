fn main() {
    proconio::input! {
        n: usize,
        x: usize,
        vp: [(usize, usize); n],
    }
    let mut sum = 0;
    for (i, (v, p)) in vp.iter().enumerate() {
        sum += v * p;
        if sum > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");

}
