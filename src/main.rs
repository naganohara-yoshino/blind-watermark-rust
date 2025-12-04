use bitvec::prelude::*;

fn main() {
    let a: String = String::from("こんにちは❗😊");
    eprintln!("a = {:?}", a);
    let a_bytes: &[u8] = a.as_bytes();
    eprintln!("a_bytes = {:?}", a_bytes);
    let bv = a_bytes.as_bits::<Lsb0>();
    for i in 0..10 {
        eprintln!("bv[i] = {:?}", bv[i]);
    }
    let r_bytes: Vec<u8> = bv.to_bitvec().into_vec();
    let b = String::from_utf8(r_bytes);
    eprintln!("b = {:?}", b);



}
