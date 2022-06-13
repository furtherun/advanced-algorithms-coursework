//bad case: 2059248508*8411782098 for mult_recur_pro
use big_int_mult::big_integer::BigInteger;
type BI = BigInteger;

fn main() {
   
 //let mut bi1 = BI::from("110");
    //let mut bi2 = BI::from("990");
    //let res = bi1.mult_recur(&mut bi2);
    //let res = bi1.mult_recur_pro(&mut bi2);
    //println!("{}", res);

    //let len: usize = 10;
    //let mut bi_x = BI::rand_init(len);
    //let mut bi_y = BI::rand_init(len);
    let mut bi_x = BI::from("2059248508");
    let mut bi_y = BI::from("8411782098");
    println!("x = {bi_x}, y = {bi_y}");
    let res1 = bi_x.mult(&mut bi_y);
    let res2 = bi_x.mult(&mut bi_y);
    let res3 = bi_x.mult_recur_pro(&mut bi_y);
    println!("{bi_x}*{bi_y}");
    println!("res1={res1}");
    println!("res2={res2}");
    println!("res3={res3}");
    println!("The result of res3 is wrong!");
}