use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }
    let theta: f64 = ((h + m / 60.0) / 6.0 - m / 30.0) * std::f64::consts::PI;
    let x2: f64 = a * a + b * b - 2.0 * a * b * theta.cos();
    println!("{}", x2.sqrt());
}
