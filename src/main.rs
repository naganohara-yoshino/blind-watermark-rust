use faer::prelude::*;
fn main() {
    let m = mat![[1.0, 2.0], [3.0, 4.0],];
    let svd_result = m.svd().unwrap();
    let u = svd_result.U();
    let v = svd_result.V();
    let mut s = svd_result.S();
    let mut ss = s * 1.0;
    ss[0] = 10.0;
    let k = u * ss * v.transpose();
    eprintln!("k = {:?}", k);
}
