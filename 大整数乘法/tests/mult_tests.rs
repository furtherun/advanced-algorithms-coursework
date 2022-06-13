#[cfg(test)]
use big_int_mult::big_integer::BigInteger;
type BI = BigInteger;

#[test]
fn test_mult() {
    assert_eq!(BI::from("1234").mult(&mut BI::from("7890")),
              BI::from("9736260"));
    assert_eq!(BI::from("0").mult(&mut BI::from("9")),
              BI::from("0"));
    assert_eq!(BI::from("1234").mult(&mut BI::from("2")),
              BI::from("2468"));
}
#[test]
fn test_add() {
    assert_eq!(BI::from("123").add(&BI::from("456")),
               BI::from("579"));
    assert_eq!(BI::from("0").add(&BI::from("9870")),
               BI::from("9870"));
    assert_eq!(BI::from("999").add(&BI::from("11")),
               BI::from("1010"));
}
//test for mult_recur overtime, but can pass by input
//#[test]
//fn test_mult_recur() {
//    assert_eq!(BI::from("0").mult_recur(&mut BI::from("9")),
//               BI::from("0"));
//    //assert_eq!(BI::from("1234").mult_recur(&mut BI::from("2")),
//    //           BI::from("2468"));
//    //assert_eq!(BI::from("0123").mult_recur(&mut BI::from("7890")),
//    //           BI::from("970470"));
//    assert_eq!(BI::from("111").mult_recur(&mut BI::from("222")),
//               BI::from("24642"));
//}
#[test]
fn test_add_hl() {
    assert_eq!(BI::from("123").add_hl(),
               BI::from("15"));
    assert_eq!(BI::from("7899").add_hl(),
               BI::from("177"));
    assert_eq!(BI::from("456").add_hl(),
               BI::from("51"));
}
#[test]
fn test_sub() {
    assert_eq!(BI::from("7890").sub(&BI::from("1234")),
               BI::from("6656"));
}
#[test]
fn test_mult_recur_pro() {
    assert_eq!(BI::from("1234").mult_recur_pro(&mut BI::from("7890")),
               BI::from("9736260"));
    assert_eq!(BI::from("0").mult_recur_pro(&mut BI::from("9")),
               BI::from("0"));
    assert_eq!(BI::from("999").mult_recur_pro(&mut BI::from("280")),
               BI::from("279720"));
}
