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

    println!("should be true {}", a == a);
    println!("should be false {}", a == b);
    println!("a + b should be equal to c {}", a + b == c);
    println!("11 - 9 should be equal to 2 {}", i - j == k);
    println!("6 - 13 should be equal to 12 {}", x - y == z);
}
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    fn new(num: i32, prime: i32) -> FieldElement {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        FieldElement { num, prime }
    }

    fn repr(self) -> String {
        format!("FieldElement_{}{}", self.prime, self.num)
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
            num = 19 + num;
        }

        Self {
            num: num % self.prime,
            prime: self.prime,
        }
    }
}
