use proconio::input;

fn main() {
    input! {
        n: u32,
        p: [(f32, f32); n],
    }
    let mut ans: i32 = 0;
    for i in 0..n {
        let (x1, y1) = p[i as usize];
        for j in 0..i {
            let (x2, y2) = p[j as usize];
            let grad = (y2 - y1) / (x2 - x1);
            if (grad >= -1.0) && (grad <= 1.0) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
