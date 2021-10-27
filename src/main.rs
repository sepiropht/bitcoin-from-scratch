use bitcoin::FieldElement;
use bitcoin::Point;
fn main() {
    let a = FieldElement::new(0, 103);
    let b: FieldElement = FieldElement::new(7, 103);
    let x: FieldElement = FieldElement::new(17, 103);
    let y: FieldElement = FieldElement::new(64, 103);

    let p = Point::new(a, b, Some(x), Some(y));

    dbg!(p);
}
