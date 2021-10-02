use bitcoin::FieldElement;

// compare 2 same fieldElement should be true
#[test]
fn compare_field_element() {
    let a = FieldElement::new(7, 13);
    let b: FieldElement = FieldElement::new(12, 13);
    assert_eq!(a, a);
    assert_ne!(a, b);
}

// add 2 fieldElement
#[test]
fn add_field_element() {
    let a = FieldElement::new(7, 13);
    let b: FieldElement = FieldElement::new(12, 13);
    let c: FieldElement = FieldElement::new(6, 13);
    assert_eq!(a + b, c);
}

// sub 2 fieldElement
#[test]
fn sub_field_element() {
    let i: FieldElement = FieldElement::new(11, 19);
    let j: FieldElement = FieldElement::new(9, 19);
    let k: FieldElement = FieldElement::new(2, 19);

    let x: FieldElement = FieldElement::new(6, 19);
    let y: FieldElement = FieldElement::new(13, 19);
    let z: FieldElement = FieldElement::new(12, 19);

    assert_eq!(i - j, k);
    assert_eq!(x - y, z);
}

// multiplication 2 fieldElement
#[test]
fn mul_field_element() {
    let o = FieldElement::new(3, 13);
    let p = FieldElement::new(12, 13);
    let q = FieldElement::new(10, 13);
    assert_eq!(o * p, q);
}

// exp  fieldElement
#[test]
fn exp_field_element() {
    let m = FieldElement::new(3, 13);
    let n = FieldElement::new(1, 13);
    assert_eq!(m.exp(3), n);
}

// div 2 fieldElement
#[test]
fn div_field_element() {
    let o = FieldElement::new(3, 13);
    let p = FieldElement::new(12, 13);
    let q = FieldElement::new(10, 13);
    assert_eq!(q / p, o);
}

// exp fieldElement with neative integer
#[test]
fn exp_neg_field_element() {
    let u = FieldElement::new(7, 13);
    let v = FieldElement::new(8, 13);
    assert_eq!(u.exp(-3), v);
}
