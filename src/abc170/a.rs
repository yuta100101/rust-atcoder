use proconio::input;

fn main() {
    input! {
        x1: i32,
        x2: i32,
        x3: i32,
        x4: i32,
        x5: i32,
    }
    println!("{}", 15 - x1 - x2 - x3 - x4 - x5);
}
