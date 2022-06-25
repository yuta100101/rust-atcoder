use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
    }
    if m == n {
        println!("Yes");
    } else {
        println!("No");
    }
}
