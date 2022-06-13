extern crate rand;
use rand::{distributions::Uniform, Rng};
use std::time::Instant;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::iter::zip;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};

const MAX_VEC_SIZE: usize = 1_00_000_000;

//const MAX_VEC_SIZE: usize = 100_000; //test size
fn main() {

    run(false);
    draw();
    
}
fn new_big_integer(len: usize) -> Option<Vec<i32>> {
    // Retruns a random big integer in vec,
    // or None if out of memory.
    // vec![5, 4, 3, 2, 1] represents the number 12345.

    if len > MAX_VEC_SIZE {
        None
    } else {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 10);

        let mut big_num: Vec<i32> = (0..len).map(|_| rng.sample(&range))
                                           .collect();

        big_num[len-1] = rng.gen_range(1..10); 
        Some(big_num)
    }
}
fn get_big_integer_string(num: &[i32]) -> String { 
    // Returns the big integer in string format,
    // vec![5, 4, 3, 2, 1] is converted into "12345".

    //let mut big_num_str = String::with_capacity(num.len());
    num.iter().rev().map(ToString::to_string).collect()
    //for n in num.iter().rev() {
    //    write!(big_num_str, "{}", n).unwrap();
    //}
    //big_num_str
}
#[test]
fn test_get_big_integer_string() {
    assert_eq!(get_big_integer_string(&[5,4,3,2,1]), "12345");
}

fn multi_big_integer(num1: &[i32], num2: &[i32]) -> Option<Vec<i32>> {
    // We assume that num1 and num2 are of the same length,
    // both of them have no leading zeros.
    // But this fuction can also deal with leading 0's situation.
    // The result must without leading 0's, but the number zero iterself.
    
    let mut big_num: Vec<i32> = vec![0; num1.len()+num2.len()];

    for i in 0..num1.len() {
        for j in 0..num2.len() {
            let mul = num1[i] * num2[j];

            let curr = i + j;
            let high = i + j + 1;
            let total = mul + big_num[curr];

            big_num[high] += total / 10;
            big_num[curr] = total % 10;
            
        }
    }

    //Removes leading 0's
    while big_num.len() > 1 && big_num[big_num.len()-1] == 0 {
        big_num.pop().unwrap();
    }

    Some(big_num)
}
#[test]
fn test_multi_big_integer() {
    assert_eq!(multi_big_integer(&[0], &[4]), Some(vec![0]));
    assert_eq!(multi_big_integer(&[1], &[4]), Some(vec![4]));
    assert_eq!(multi_big_integer(&[2,3], &[2,0]), Some(vec![4,6]));
    assert_eq!(multi_big_integer(&[2,1], &[8,1]), Some(vec![6,1,2]));
    assert_eq!(multi_big_integer(&[4,2,0,1], &[9,8,7,1]),
              Some(vec![6,3,9,1,3,8,1]));
                                                                                
    assert_eq!(multi_big_integer(&[4,2,2,1,3],
                                           &[3,0,2,2,1]),
           Some(vec![2,7,4,6,2,0,1,8,3]));

    assert_eq!(multi_big_integer(&[4,2,2,1,3,7,9,6,5,5],
                                 &[3,0,2,2,1,6,3,2,0,6],),
               Some(vec![2,7,4,6,2,5,6,1,3,8,6,9,0,0,9,9,4,5,3,3]));
}
fn add_big_integer(num1: &[i32], num2: &[i32]) -> Option<Vec<i32>> {
    //We assume that the length of num1 is not less than that of num2.
    //Example:
    //num1: [4, 2, 0, 1] represents "1024", num2: [9, 8, 7] represents "789".

    if num2.len() > num1.len() {
        return add_big_integer(num2, num1);
    }

    let mut big_num = num1.to_vec();
    big_num.push(0); //add 0 at highest postion

    let mut carry = 0;

    for i in 0..big_num.len() {
        if i < num2.len() {
            big_num[i] += num2[i] + carry;
        } else {
            big_num[i] += carry;
        }
        carry = big_num[i] / 10;
        big_num[i] = big_num[i] % 10;
            
    }
    //Removes leading 0's
    while big_num.len() > 1 && big_num[big_num.len()-1] == 0 {
        big_num.pop().unwrap();
    }

    Some(big_num)
}
#[test]
fn test_add_big_integer() {
    assert_eq!(add_big_integer(&[0], &[0]),
               Some(vec![0]));

    assert_eq!(add_big_integer(&[4, 2, 0, 1], &[9, 8, 7]),
               Some(vec![3, 1, 8, 1]));
    assert_eq!(add_big_integer(&[9, 8, 7], &[4, 2, 0, 1]),
              Some(vec![3, 1, 8, 1]));
}
fn shift_big_integer(num: &[i32], shift_size: usize) -> Option<Vec<i32>> {
    // A helper function for recursion approach deal with big integer
    // shift.

    //let mut shift_num = num.to_vec();
    //shift_num.reverse();
    //shift_num.append(&mut vec![0; shift_size]);
    //shift_num.reverse();
    //Some(shift_num)

    let mut shift_num = vec![0; shift_size];
    shift_num.extend_from_slice(num);

    Some(shift_num)
}
fn multi_big_integer_recursion(num1: &[i32], num2: &[i32]) -> Option<Vec<i32>> {
    // We assume that num1 and num2 are of the same length,
    // both of them have no leading zeros.
    // But it is powerful to deal with unequal lengths and leading 0's
    // num1 = a1 * 10^{n/2} + a0, num2 = b1 * 10^{m/2} + b0
    // num1 * num2 = a1b1*10^{n/2+m/2} + a0b1*10^{m/2} + a1b0*10^{n/2} + a0b0

    if num1.len() == 1 || num2.len() == 1 {
        return multi_big_integer(&num1, &num2);
    } 
    
    let a0 = &num1[..num1.len()/2]; //low part of num1
    let a1 = &num1[num1.len()/2..]; //high part of num1
    let b0 = &num2[..num2.len()/2]; //low part of num2
    let b1 = &num2[num2.len()/2..]; //high part of num2
    

    match (multi_big_integer_recursion(a0, b0),
           multi_big_integer_recursion(a1, b1),
           multi_big_integer_recursion(a0, b1),
           multi_big_integer_recursion(a1, b0)) {
        (Some(a0b0), Some(a1b1), Some(a0b1), Some(a1b0)) => {
            let a1b1_shift = shift_big_integer(&a1b1, 
                                       num1.len()/2+num2.len()/2).unwrap();
            let a0b1_shift = shift_big_integer(&a0b1, 
                                       num2.len()/2).unwrap();
            let a1b0_shift = shift_big_integer(&a1b0, 
                                       num1.len()/2).unwrap();
            //println!("{:?}, {:?}, {:?}, {:?}", a1b1_shift, a0b1_shift, a1b0_shift, a0b0);

            add_big_integer(&add_big_integer(&a1b1_shift, &a0b0).unwrap(),
                            &add_big_integer(&a0b1_shift, &a1b0_shift).unwrap())
        }
        _ => None,
    }
}
#[test]
fn test_multi_big_integer_recursion() {
    assert_eq!(multi_big_integer_recursion(&[1], &[1]), Some(vec![1]));
    assert_eq!(multi_big_integer_recursion(&[1,0,0,1], &[2,0,0,2]),
              Some(vec![2,0,0,4,0,0,2]));
    
    assert_eq!(multi_big_integer_recursion(&[2,1], &[8,1]), Some(vec![6,1,2]));
    assert_eq!(multi_big_integer_recursion(&[2,2], &[2,0]), Some(vec![4,4]));
    assert_eq!(multi_big_integer_recursion(&[4,2,0,1], &[9,8,7,1]),
              Some(vec![6,3,9,1,3,8,1]));

    assert_eq!(multi_big_integer_recursion(&[4,2,2],
                                           &[3,0,2]),
           Some(vec![2,7,4,5,4]));

    assert_eq!(multi_big_integer_recursion(&[4,2,2,1,3],
                                           &[3,0,2,2,1]),
           Some(vec![2,7,4,6,2,0,1,8,3]));

    assert_eq!(multi_big_integer_recursion(&[4,2,2,1,3,7,9,6,5,5],
                                           &[3,0,2,2,1,6,3,2,0,6],),
           Some(vec![2,7,4,6,2,5,6,1,3,8,6,9,0,0,9,9,4,5,3,3]));
}

fn sub_big_integer(num1: &[i32], num2: &[i32]) -> Option<Vec<i32>> {
    // We assure that num1 is not less than num2, so the result
    // will be a positive number.
    
    if num1.len() < num2.len() {
        return None;
    }

    let mut big_num = num1.to_vec();

    for i in 0..num2.len() {
        if big_num[i] < num2[i] {
            big_num[i+1] -= 1;
            big_num[i] += 10;
        }
        big_num[i] -= num2[i];
    }
    
    for i in num2.len()..big_num.len() {
        if big_num[i] < 0 {
            big_num[i+1] -= 1;
            big_num[i] += 10;
        }
    }

    //Removes leading 0's, and at lest keep one digit
    //Example:
    //[9, 8] represents "89", 89 - 89 = 0,
    //We should return [0], instead of [].
    while big_num.len() > 1 && big_num[big_num.len()-1] == 0 {
        big_num.pop().unwrap();
    }
    
    Some(big_num)
}
#[test]
fn test_sub_big_integer() {
    assert_eq!(sub_big_integer(&[9,8], &[9,8]), Some(vec![0]));

    assert_eq!(sub_big_integer(&[8], &[8]), Some(vec![0]));

    assert_eq!(sub_big_integer(&[4, 3, 2, 1], &[9, 8, 7]), Some(vec![5, 4, 4]));

    assert_eq!(sub_big_integer(&[1,0,0,1], &[9,9]), Some(vec![2,0,9]));
}
fn multi_big_integer_recursion_plus(num1: &[i32], num2: &[i32]) -> Option<Vec<i32>> {
    // The num1 and num2 must be of the same length,
    // both of them have no leading zeros.
    // num1 = a1 * 10^{n/2} + a0, num2 = b1 * 10^{n/2} + b0
    // Idea1:
    // num1 * num2 = a0b0 + (a0b1+a1b0) * 10^{n/2} + a1b1 * 10^{n/2+n/2}
    //             = a0b0 + ((a0-a1)(b1-b0) + a0b0 + a1b1) * 10^{n/2} 
    //                    + a1b1 * 10^{n/2+n/2}
    // In this expression, the signs of (a0-a1) and (b1-b0) are all not sure.
    //
    // Idea2:
    // num1 * num2 = a0b0 + (a0b1+a1b0) * 10^{n/2} + a1b1 * 10^{n/2}
    //             = a0b0 + ((a0+a1)(b0+b1) - a0b0 - a1b1) * 10^{n/2} 
    //                    + a1b1 * 10^{n/2}
    //             = a0b0 + ((a0+a1)(b0+b1) - (a0b0 + a1b1)) * 10^{n/2} 
    //                    + a1b1 * 10^{n/2+n/2}
    // It is easy to handle the sign, (a0+a1)(b0+b1) must be greater than
    // the sum of a0b0 and a1b1, since a0b1+a1b0 is always a postive number. 

    if num1.len() == 1 || num2.len() == 1 {
        return multi_big_integer(&num1, &num2);
    } 
    
    let a0 = &num1[..num1.len()/2]; //low part of num1
    let a1 = &num1[num1.len()/2..]; //high part of num1
    let b0 = &num2[..num2.len()/2]; //low part of num2
    let b1 = &num2[num2.len()/2..]; //high part of num2

    //println!("a0 = {:?}, a1 = {:?}, b0 = {:?}, b1 = {:?}", a0, a1, b0, b1);                                                                              

    let a0_add_a1 = add_big_integer(a0, a1).unwrap();
    let b0_add_b1 = add_big_integer(b0, b1).unwrap();
    
    //println!("a0+a1 = {:?}, b0+b1 = {:?}", a0_add_a1, b0_add_b1);                                                                              
    
    match (multi_big_integer_recursion(&a0_add_a1, &b0_add_b1),
           multi_big_integer_recursion(a0, b0),
           multi_big_integer_recursion(a1, b1)) {
        (Some(product_ab), Some(a0b0), Some(a1b1)) => {
            let add_ab = add_big_integer(&a0b0, &a1b1).unwrap();
            let mid = sub_big_integer(&product_ab, &add_ab).unwrap();

            //println!("add_ab = {:?}, mid = {:?}", add_ab, mid);

            let a1b1_shift =shift_big_integer(&a1b1, 
                                          num1.len()/2+num2.len()/2).unwrap();
            let mid_shift = shift_big_integer(&mid,
                                          num2.len()/2).unwrap();
            
            //println!("high = {:?}, mid = {:?}, low = {:?}", a1b1_shift, mid_shift, a0b0);                                                                              

            add_big_integer(&add_big_integer(&a1b1_shift, &a0b0).unwrap(),
                            &mid_shift)

        }
        _ => None,
    }
}

#[test]
fn test_multi_big_integer_recursion_plus() {
    assert_eq!(multi_big_integer_recursion_plus(&[1], &[1]), Some(vec![1]));         
    assert_eq!(multi_big_integer_recursion_plus(&[1,0,0,1], &[2,0,0,2]),
              Some(vec![2,0,0,4,0,0,2]));
    assert_eq!(multi_big_integer_recursion_plus(&[2,1], &[8,1]), Some(vec![6,1,2]));
    assert_eq!(multi_big_integer_recursion_plus(&[2,2], &[2,0]), Some(vec![4,4]));
    assert_eq!(multi_big_integer_recursion_plus(&[4,2,0,1], &[9,8,7,1]),
              Some(vec![6,3,9,1,3,8,1]));

    assert_eq!(multi_big_integer_recursion_plus(&[4,2,2], &[3,0,2]),
              Some(vec![2,7,4,5,4]));
    assert_eq!(multi_big_integer_recursion_plus(&[4,2,2,1,3], &[3,0,2,2,1]),
              Some(vec![2,7,4,6,2,0,1,8,3]));
    assert_eq!(multi_big_integer_recursion_plus(&[4,2,2,1,3,7,9,6,5,5],
                                           &[3,0,2,2,1,6,3,2,0,6],),            
              Some(vec![2,7,4,6,2,5,6,1,3,8,6,9,0,0,9,9,4,5,3,3]));                
}


fn get_multi_time(num1: &[i32], num2: &[i32],
                  multi_func: fn(&[i32], &[i32]) -> Option<Vec<i32>>) -> f64 {

    if num1.len() < 1000 {
        let now = Instant::now();
        for _ in 0..1000 {
            multi_func(num1, num2);
        }
        
        return now.elapsed().as_millis() as f64 / 1000.0;
    }

    let now = Instant::now();
    multi_func(num1, num2);
    now.elapsed().as_millis() as f64 
}

fn run(is_display: bool) {

    let mut len = 10;
    let multi_func_list = [multi_big_integer, multi_big_integer_recursion,
                       multi_big_integer_recursion_plus];

    let mut file = File::create("datas/data1.csv").expect("create failed"); 

    loop {
        match (new_big_integer(len), new_big_integer(len)) {
            (Some(num1), Some(num2)) => {

                println!("Big integer length = {len} generated.");
                if is_display {
                    println!("num1 is {}", get_big_integer_string(&num1));
                    println!("num2 is {}", get_big_integer_string(&num2));
                }
                
                writeln!(file, "{},{},{},{}", len,
                    get_multi_time(&num1, &num2, multi_func_list[0]),
                    get_multi_time(&num1, &num2, multi_func_list[1]),
                    get_multi_time(&num1, &num2, multi_func_list[2]),).unwrap();
                
                println!("Big integer length = {len} finished.");
                //for func in multi_func_list {
                //    write!(file, "{},", get_multi_time(&num1, &num2, func)).unwrap();
                //}
            },
            _ => {
                println!("Out of memory!");
                break;
            }

        };

        len *= 10;
    };

}

fn draw() {
    let file = File::open("datas/data1.csv")
        .expect("open failed");

    let mut x_axis: Vec<f64> = vec![];
    let mut y1_axis: Vec<f64> = vec![];
    let mut y2_axis: Vec<f64> = vec![];
    let mut y3_axis: Vec<f64> = vec![];
                                                              
    for line in BufReader::new(file).lines() {
        let info = line.unwrap();
        let info_list: Vec<&str> = info.split(',').collect();
                                                              
        x_axis.push(info_list[0].parse::<f64>()
                                .unwrap() .log10());
        y1_axis.push(info_list[1].parse::<f64>()
                                 .unwrap().log10()); 
        y2_axis.push(info_list[2].parse::<f64>()
                                 .unwrap().log10()); 
        y3_axis.push(info_list[3].parse::<f64>()
                                 .unwrap().log10()); 
    }
                                                              
    println!("{:?}, {:?}, {:?}, {:?}",
             x_axis, y1_axis,
             y2_axis,
             y3_axis);
    
    let data1: Vec<(_, _)>  = zip(x_axis.clone(), y1_axis.clone()).collect();
    let data2: Vec<(_, _)>  = zip(x_axis.clone(), y2_axis.clone()).collect();
    let data3: Vec<(_, _)>  = zip(x_axis.clone(), y3_axis.clone()).collect();
    
    println!("{:?}, {:?}, {:?}", data1, data2, data3);

    let s1: Plot = Plot::new(data1).point_style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("#DD3355")
    ).legend("multi_big_integer".to_string()); // and a custom colour                                                    
    let s2: Plot = Plot::new(data2).point_style(                               
        PointStyle::new()
            .marker(PointMarker::Cross) // setting the marker to be a square
            .colour("#35C788"),
    ).legend("multi_big_integer_recursion".to_string()); // and a custom colour
    let s3: Plot = Plot::new(data3).point_style(                              
        PointStyle::new()
            .marker(PointMarker::Circle) // setting the marker to be a square
            .colour("#00A4FF"),
    ).legend("multi_big_integer_recursion_plus".to_string()); // and a custom colour

    // The 'view' describes what set of data is drawn         
    let view = ContinuousView::new()
        .add(s1)
        .add(s2)
        .add(s3)
        .x_range(0., 6.)
        .y_range(-2., 9.)
        .x_label("log of length")
        .y_label("log of time");
                                                              
    // A page with a single view is then saved to an SVG file
    Page::single(&view).save("results/result1.svg").unwrap();
}
