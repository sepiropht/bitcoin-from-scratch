use bitcoin::Point;

// raise exception when point is not on the curve
#[test]
#[should_panic(expected = "(-1, -2) is not on the curve")]
fn not_in_curve() {
    Point::new(5, 7, Some(-1), Some(-2));
}

// same point should be equal
#[test]
fn eq() {
    assert_eq!(Point::new(5, 7, Some(-1), Some(-1)), Point::new(5, 7, Some(-1), Some(-1)));
}
