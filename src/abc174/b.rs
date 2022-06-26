use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        v: [(f64, f64); n],
    }
    let mut ans: i64 = 0;
    for (x, y) in v {
        if x.hypot(y) <= d {
            ans += 1;
        }
    }
    println!("{}", ans);
}
