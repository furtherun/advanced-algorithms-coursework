//use big_int_mult::big_integer::*;

use big_int_mult::big_integer::BigInteger;
type BI = BigInteger;
fn main() {

    let len: usize = 10;
    let mut bi_x = BI::rand_init(len);
    let mut bi_y = BI::rand_init(len);
    println!("x = {bi_x}, y = {bi_y}");
    let res1 = bi_x.mult(&mut bi_y);
    let res2 = bi_x.mult(&mut bi_y);
    let res3 = bi_x.mult_recur_pro(&mut bi_y);
    println!("{bi_x}*{bi_y}");
    println!("res1={res1}");
    println!("res2={res2}");
    println!("res3={res3}");
}