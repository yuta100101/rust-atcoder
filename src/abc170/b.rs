use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }
    if y % 2 != 0 || y < 2 * x || y > 4 * x {
        println!("No");
    } else {
        println!("Yes");
    }
}
