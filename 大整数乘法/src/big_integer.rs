#![allow(dead_code)]
const CAPACITY: usize = 1_00_000_000; 
extern crate rand;

#[derive(Debug)]
pub struct BigInteger {

    pub start: usize,
    pub len: usize,
    pub nums: Vec::<i32>,
}

impl BigInteger {
    pub fn new() -> BigInteger {
        BigInteger {
            start: 0,
            len: 1,
            nums: vec![0; CAPACITY],
        }
    }
    pub fn init(len: usize) -> BigInteger {
        BigInteger {
            start: 0,
            len,
            nums: vec![0; CAPACITY],
        }
    }
    pub fn rand_init(len: usize) -> BigInteger {
        let mut bi = BigInteger::init(len);

        use rand::distributions::{Distribution, Uniform};

        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..10);

        for i in 0..len {
            bi.nums[i] = die.sample(&mut rng); 
        }

        bi
    }
    pub fn from(s: &str) -> BigInteger {
        let mut bi = BigInteger::init(s.len());

        for (i, c) in s.chars().rev().enumerate() {
            bi.nums[i] = c.to_digit(10).unwrap() as i32;
        }

        bi
    }
    //pub fn is_empty(&self) -> bool {
    //    self.len == 0
    //}
    pub fn to_string(&self) -> String {
        self.nums.get(self.start..self.start+self.len).unwrap()
            .iter().rev()
            .map(|i| i.to_string()).collect::<String>()
    }
    pub fn mult(&mut self, other: &mut BigInteger) -> BigInteger {
        let mut bi = BigInteger::new(); //0

        if *self == bi || *other == bi {
            return bi;
        }
        
        bi.set_start_len(self.start+other.start, 
                         self.len+other.len);
        
        for i in self.start..self.start+self.len {
            for j in other.start..other.start+other.len {
                let mul = self.nums[i] * other.nums[j];

                let curr = i + j;
                let high = i + j + 1;
                let total = mul + bi.nums[curr];

                bi.nums[high] += total / 10;
                bi.nums[curr] = total % 10;
            }
        }

        bi.remove_leading_zero();

        bi
    }
    fn remove_leading_zero(&mut self) {
        while self.len > 1 &&
              self.nums[self.start+self.len-1] == 0
            {
                self.len -= 1;  
            }
    }
    fn set_start_len(&mut self, start:usize, len: usize) {
        self.start = start;
        self.len = len;
    }
    pub fn sub(&mut self, other: &BigInteger) -> BigInteger {
        //Assure self is larger than other
        if self.len < other.len {
            panic!("sub length error");
        }

        let mut bi = BigInteger::init(self.len);

        for i in 0..bi.len {
            if self.nums[i] < other.nums[i] {
                self.nums[i+1] -= 1;
                self.nums[i] += 10;
            }

            bi.nums[i] = self.nums[i] - other.nums[i];
        }

        bi.remove_leading_zero();

        bi
    }
    pub fn shift(&mut self, index: usize) -> BigInteger {
        let mut bi = BigInteger::init(index+self.len);

        for i in 0.. self.len {
            bi.nums[index+i] = self.nums[self.start+i]; 
        }

        bi
    }
    pub fn add(&self, other: &BigInteger) -> BigInteger
    {
        if self.len < other.len {
            return other.add(self);
        }

        let mut bi = BigInteger::init(self.len+1);

        let mut carry = 0;

        for i in 0..bi.len {
            let total = self.nums[self.start+i] + 
                        other.nums[other.start+i] + carry;

            bi.nums[i] = total % 10;
            carry = total / 10;

        }
        bi.remove_leading_zero();

        bi
    }
    pub fn add_hl(&self) -> BigInteger {
        //high part: [start+len/2, start+len]
        //low part:  [start, start+len/2]
                                                             
        let mut carry = 0;
                                                             
        //high part is not shorter than low part
        let len = self.len / 2;
        
        let mut bi = BigInteger::init(len+1);

        for i in 0..len {
            let total = self.nums[self.start+len+i] +
                        self.nums[self.start+i] + carry;
                                                             
            bi.nums[i] = total % 10;
            carry = total / 10;
        }
                                                             
        bi.nums[len] = 
            if len == self.len - len {
                0
            } else {
                self.nums[self.start+self.len-1]
            }  + 
            if carry != 0 {
                carry
            } else {
                0
            };
                                                             
        bi.remove_leading_zero();
                                                             
        bi
    }
    pub fn mult_recur(&mut self, other: &mut BigInteger) 
        -> BigInteger
    {
        if self.len == 1 || other.len == 1 {
            return self.mult(other);
        }

        let l_len1 = self.len/2;
        let start1 = self.start;
        let l_len2 = other.len/2;
        let start2 = other.start;

        let h_len1 = self.len - l_len1;
        let h_len2 = other.len - l_len2;

        self.set_start_len(start1+l_len1, h_len1);
        other.set_start_len(start2+l_len2, h_len2);
        //println!("self = {self}, other = {other}");
        let mut hh = self.mult_recur(other);

        //self.set_start_len(start1+len1/2, len1-len1/2);
        other.set_start_len(start2, l_len2);
        let hl = self.mult_recur(other);

        self.set_start_len(start1, l_len1);
        let mut ll = self.mult_recur(other);

        other.set_start_len(start2+l_len2, h_len2);
        let lh = self.mult_recur(other);

        let mut mid = hl.add(&lh);

        hh.set_start_len(0, hh.start+hh.len);
        //hl.set_start_len(0, hl.start+hl.len);
        //lh.set_start_len(0, lh.start+lh.len);
        ll.set_start_len(0, ll.start+ll.len);
        //println!("hh={hh}, hl={hl}, lh = {lh}, ll={ll}");
        mid.set_start_len(0, mid.start+mid.len);

        //let bi = hh.add(&hl).add(&lh).add(&ll);
        //println!("res = {bi}");
        //hh.add(&hl).add(&lh).add(&ll)
        hh.add(&mid).add(&ll)

    }

    pub fn mult_recur_pro(&mut self, other: &mut BigInteger)
        -> BigInteger
    {
        if self.len == 1 || other.len == 1 {
            return self.mult(other);
        }
                                                                         
        let len1 = self.len;
        let start1 = self.start;
        let len2 = other.len;
        let start2 = other.start;

        //Multiply hh
        self.set_start_len(start1+len1/2,len1-len1/2);
        other.set_start_len(start2+len2/2,len2-len2/2);
        //println!("self = {self}, other = {other}");
        let mut hh = self.mult_recur_pro(other);
        //println!("hh = {hh}");
        
        //Multiply ll
        self.set_start_len(start1, len1/2);
        other.set_start_len(start2, len2/2);
        let ll = self.mult_recur_pro(other);
        //println!("ll={ll}");
                                                                         
        //Recover original data
        self.set_start_len(start1, len1);
        other.set_start_len(start2, len2);

        let mut hl1 = self.add_hl();
        let mut hl2 = other.add_hl();
        //println!("hl1={hl1}, hl2={hl2}");
        let mut mid = hl1.mult_recur_pro(&mut hl2).sub(&hh.add(&ll));

        //println!("mid = {mid}");
        
        mid = mid.shift(len2/2);
        hh = hh.shift(len1/2+len2/2);

        //println!("hh={hh},mid={mid},ll={ll}");
        //let ret = hh.add(&mid).add(&ll);
        //println!("{ret}");

        hh.add(&mid).add(&ll)
    }
}

use std::fmt;

impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

use std::cmp;

impl cmp::PartialEq for BigInteger {
    fn eq(&self, other: &BigInteger) -> bool {
        if self.len != other.len {
            return false;
        }

        for idx in 0..self.len {
            if self.nums[self.start+idx] != 
                other.nums[other.start+idx] 
            {
                return false;
            }
        }
        true
    }
}
