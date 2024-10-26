fn main() {
    proconio::input! {
        s: String,
    }
    let mut c = s.chars().collect::<Vec<char>>();

    if c[0] == c[1] && c[1] == c[2] {
        println!("Won");
    } else {
        println!("Lost");
    }
}
