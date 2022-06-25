use proconio::input;

fn main() {
    input! {
        d: i32,
        t: i32,
        s: i32,
    }
    if (d - 1) / s + 1 <= t {
        println!("Yes");
    } else {
        println!("No");
    }
}
