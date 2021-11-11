use std::vec;

use bitcoin::FieldElement;
use bitcoin::Point;

#[test]
fn rmul() {
    let prime = 223;
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);

    let multiplications = vec![
        (2, 192, 105, 49, 71),
        (2, 143, 98, 64, 168),
        (2, 47, 71, 36, 111),
        (4, 47, 71, 194, 51),
        (8, 47, 71, 116, 55),
    ];
    for (s, x1_raw, y1_raw, x2_raw, y2_raw) in multiplications.iter() {
        let s = *s as u128;
        let x1 = FieldElement::new(*x1_raw, prime);
        let y1 = FieldElement::new(*y1_raw, prime);
        let p1 = Point::new(a, b, Some(x1), Some(y1));

        let x2 = FieldElement::new(*x2_raw, prime);
        let y2 = FieldElement::new(*y2_raw, prime);
        let p2 = Point::new(a, b, Some(x2), Some(y2));

        assert_eq!(p1 * s, p2);
    }
}
