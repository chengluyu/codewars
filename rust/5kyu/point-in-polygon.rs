// type Point = (f32, f32);

// https://www.codewars.com/kata/point-in-polygon-1/train/rust
// from https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule

fn point_in_poly(poly: &[Point], (x, y): Point) -> bool {
    let mut ans = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        if (poly[i].1 > y) != (poly[j].1 > y) && (x < poly[i].0 + (poly[j].0 - poly[i].0) * (y - poly[i].1) / (poly[j].1 - poly[i].1)) {
            ans = !ans;
        }
        j = i;
    }
    ans
}
