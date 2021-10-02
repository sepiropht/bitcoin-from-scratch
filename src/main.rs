fn main() {
    let a = FieldElement::new(7, 13);
    let b: FieldElement = FieldElement::new(12, 13);
    let c: FieldElement = FieldElement::new(6, 13);

    let i: FieldElement = FieldElement::new(11, 19);
    let j: FieldElement = FieldElement::new(9, 19);
    let k: FieldElement = FieldElement::new(2, 19);

    let x: FieldElement = FieldElement::new(6, 19);
    let y: FieldElement = FieldElement::new(13, 19);
    let z: FieldElement = FieldElement::new(12, 19);

    let o = FieldElement::new(3, 13);
    let p = FieldElement::new(12, 13);
    let q = FieldElement::new(10, 13);


    let m = FieldElement::new(3, 13);
    let n = FieldElement::new(1, 13);

    let u = FieldElement::new(7, 13);
    let v = FieldElement::new(8, 13);



    println!("should be true {}", a == a);
    println!("should be false {}", a == b);
    println!("a + b should be equal to c {}", a + b == c);
    println!("11 - 9 should be equal to 2 {}", i - j == k);
    println!("6 - 13 should be equal to 12 {}", x - y == z);
    println!("3 * 12 should be equal to 10 {}", o * p == q);
    println!("3 exp 3 should be equal to 1 {}", m.exp(3) == n);
    println!("10 / 12 should be equal to 3 {}", q / p == o);
    println!("7 exp -3 should be equal to 8 {}", u.exp(-3) == v);
}
use std::ops::{Add, Sub, Mul, Div};
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
struct FieldElement {
    num: i64,
    prime: i64,
}

impl FieldElement {
    fn new(num: i64, prime: i64) -> FieldElement {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        FieldElement { num, prime }
    }

    fn repr(self) -> String {
        format!("FieldElement_{}{}", self.prime, self.num)
    }

    fn exp(self, mut exponent: i64) -> FieldElement {
        if exponent < 0 {
            exponent = self.prime - 1  + (exponent % (self.prime - 1));
        }
      
        let num = self.num.pow(exponent.try_into().unwrap()) % self.prime;
        Self {
           num,
           prime: self.prime
        }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different field");
        }
        Self {
            num: (self.num + other.num) % self.prime,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot sub two numbers in different field");
        }
        let mut num = self.num - other.num;
        if num < 0 {
            num = self.prime + num;
        }

        Self {
            num: num % self.prime,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot mutliplicate two numbers in different field");
        }
        let num = self.num * other.num;

        Self {
            num: num % self.prime,
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot divise two numbers in different field");
        }
        if other.prime == 0 {
            panic!("Cannot divise by 0");
        }
        let exp : u32 = (self.prime - 2).try_into().unwrap();
        let num = self.num * other.num.pow(exp);

        Self {
            num: num % self.prime,
            prime: self.prime,
        }
    }
}

