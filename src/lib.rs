use std::convert::TryInto;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct FieldElement {
    pub num: i64,
    pub prime: i64,
}

impl FieldElement {
    pub fn new(num: i64, prime: i64) -> FieldElement {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        FieldElement { num, prime }
    }

    pub fn repr(self) -> String {
        format!("FieldElement_{}{}", self.prime, self.num)
    }

    pub fn exp(self, mut exponent: i64) -> FieldElement {
        if exponent < 0 {
            exponent = self.prime - 1 + (exponent % (self.prime - 1));
        }

        let num = self.num.pow(exponent.try_into().unwrap()) % self.prime;
        Self {
            num,
            prime: self.prime,
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
        let exp: u32 = (self.prime - 2).try_into().unwrap();
        let num = self.num * other.num.pow(exp);

        Self {
            num: num % self.prime,
            prime: self.prime,
        }
    }
}
