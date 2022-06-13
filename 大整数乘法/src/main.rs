mod big_integer;
use big_integer::*;

use std::time::Instant;
use std::fs::File;
use std::io::Write;

const TEST_SIZE:usize = 1_00_000_000;
fn main() {
    run(false);
}

fn run(is_display: bool) { 
    let mut len:usize = 10;

    let mut file = File::create("datas/data2.csv")
        .expect("create failed"); 

    type BI = BigInteger;
    loop {
        if len > TEST_SIZE {
            panic!("out of memory");
        }
        let mut x = BI::rand_init(len);
        let mut y = BI::rand_init(len);
        if is_display {
            println!("x is {x}");
            println!("y is {y}");
        }

        println!("Big integer length = {} generated.", len);

        let t1 = get_mult_time(&mut x, &mut y, 1);
        let t2 = get_mult_time(&mut x, &mut y, 2);
        let t3 = get_mult_time(&mut x, &mut y, 3);
        
        println!("mult takes {} ms, \
            mult_recur takes {} ms, \
            mult_recur_pro takes {} ms", t1, t2, t3);

        writeln!(file, "{},{},{},{}", len, t1, t2, t3).unwrap();

        println!("Big integer length = {} finished.", len);
        len *= 10;
    };

}
fn get_mult_time(bi1: &mut BigInteger, bi2: &mut BigInteger,
    tag: i32) -> f64
{
    if bi1.len < 1000 {
        let now = Instant::now();
        for _ in 0..1000 {
            match tag {
                3 => bi1.mult_recur_pro(bi2), 
                2 => bi1.mult_recur(bi2), 
                1 => bi1.mult(bi2), 
                _ => panic!("no a tag"),
            };
        }
        
        return now.elapsed().as_millis() as f64 / 1000.0;
    }

    let now = Instant::now();

    match tag {
        3 => bi1.mult_recur_pro(bi2),
        2 => bi1.mult_recur(bi2),
        1 => bi1.mult(bi2),
        _ => panic!("no a tag"),
    };

    now.elapsed().as_millis() as f64 
}
