fn main() {
    let a = FieldElement::new(7, 13);
    let b: FieldElement = FieldElement::new(6, 13);
    println!("should be true {}", a == a);
    println!("should be false {}", a == b);
}
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
