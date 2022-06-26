use proconio::input;

fn main() {
    input! {
        x: [f64],
    }
    println!("{}", x.iter().fold(0.0, |sum, a| sum + a.abs()));
    println!("{}", x.iter().fold(0.0, |sum, a| sum + a * a).sqrt());
    println!("{}", x.iter().fold(0.0/0.0, |m, v| v.abs().max(m)));
}
