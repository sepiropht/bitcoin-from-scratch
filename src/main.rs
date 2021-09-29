fn main() {
    let a = FieldElement::new(7, 13);
    let b: FieldElement = FieldElement::new(12, 13);
    let c: FieldElement = FieldElement::new(6, 13);

    println!("should be true {}", a == a);
    println!("should be false {}", a == b);
    println!("a + b should be equal to c {}", a + b == c);
}
use std::ops::Add;

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
