use proconio::input;

fn main() {
    input! {
        k: i32,
    }
    input! {
        a: i32,
        b: i32,
    }
    if b / k - (a - 1) / k > 0 {
        println!("OK");
    } else {
        println!("NG");
    }
}
