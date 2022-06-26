use proconio::input;

fn main() {
    input! {
        vector: [i64],
    }
    let mut now: i64 = vector[0];
    let mut ans: i64 = 0;
    for a in vector {
        if a < now {
            ans += now - a;
        } else {
            now = a;
        }
    }
    println!("{}", ans);
}
