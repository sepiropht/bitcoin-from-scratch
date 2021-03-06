use std::convert::TryInto;
use std::ops::{Add, Div, Mul, Sub};
extern crate num_bigint;
use num_bigint::{BigInt, Sign};
use num_traits::{pow, ToPrimitive};

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

    pub fn pow(self, exponent: i64) -> FieldElement {
        let exponent = exponent % (self.prime - 1);

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
            num += self.prime;
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

impl Mul<i64> for FieldElement {
    type Output = Self;

    fn mul(self, scalar: i64) -> Self {
        let mut product = Self {
            num: 0,
            prime: self.prime,
        };
        for _ in 0..scalar {
            product = product + self;
        }

        product
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

        let x = BigInt::new(Sign::Plus, vec![other.num.try_into().unwrap()]);
        let res =
            (self.num * (pow(x, (self.prime - 2).try_into().unwrap()) % self.prime)) % self.prime;
        let res = truncate_biguint_to_u32(&res);
        Self {
            num: res.try_into().unwrap(),
            prime: self.prime,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
    pub a: FieldElement,
    pub b: FieldElement,
}

impl Point {
    pub fn new(
        a: FieldElement,
        b: FieldElement,
        x: Option<FieldElement>,
        y: Option<FieldElement>,
    ) -> Self {
        match (x, y) {
            (_, None) => Self { x, y: None, a, b },
            (None, _) => Self { x: None, y, a, b },
            _ => {
                let y = y.unwrap();
                let x = x.unwrap();

                if y.pow(2) != x.pow(3) + a * x + b {
                    panic!("({:?}, {:?}) is not on the curve", x, y);
                }
                Self {
                    x: Some(x),
                    y: Some(y),
                    a,
                    b,
                }
            }
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points are not on the same curve");
        }
        if self.x == None {
            return other;
        }
        if other.x == None {
            return self;
        }

        if self.x == other.x && self.y != other.y {
            return Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            };
        }

        if self == other && self.y == None {
            return Self {
                x: None,
                y: None,
                a: other.a,
                b: other.b,
            };
        }

        if self.x != other.x {
            let s = (other.y.unwrap() - self.y.unwrap()) / (other.x.unwrap() - self.x.unwrap());
            let x = s.pow(2) - self.x.unwrap() - other.x.unwrap();
            let y = s * (self.x.unwrap() - x) - self.y.unwrap();
            return Self {
                x: Some(x),
                y: Some(y),
                a: other.a,
                b: other.b,
            };
        }
        if self == other && self.y == Some(self.x.unwrap() * 0) {
            return Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            };
        }
        let s = (self.x.unwrap().pow(2) * 3 + self.a) / (self.y.unwrap() * 2);
        let x = s.pow(2) - self.x.unwrap() * 2;
        let y = s * (self.x.unwrap() - x) - self.y.unwrap();
        return Self {
            x: Some(x),
            y: Some(y),
            a: other.a,
            b: other.b,
        };
    }
}
impl Mul<u128> for Point {
    type Output = Self;
    fn mul(self, scalar: u128) -> Self {
        let mut product = Point::new(self.a, self.b, None, None);
        for _ in 0..scalar {
            product = product + self;
        }
        product
    }
}

fn truncate_biguint_to_u32(a: &BigInt) -> u32 {
    use std::u32;
    let mask = BigInt::from(u32::MAX);
    (a & mask).to_u32().unwrap()
}
