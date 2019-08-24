use std::f64::consts::PI;

fn rnd10(f: f64) -> f64 { (f * 1e10).round() / 1e10 }

fn iter_pi(epsilon: f64) -> (i32, f64) {
    let mut result: f64 = 0f64;
    let mut sign: f64 = 1f64;
    let mut iter: i32 = 0;
    while (result * 4f64 - PI).abs() > epsilon {
        result += sign / (iter * 2 + 1) as f64;
        sign *= -1f64;
        iter += 1;
    }
    (iter, rnd10(result * 4f64))
}
