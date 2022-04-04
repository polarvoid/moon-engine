use moon_engine::collider::*;
use moon_engine::Point;

#[test]
fn point_and_itself() {
    let p1 = Point::zeros();
    let p2 = Point::zeros();
    assert_eq!(p1, p2)
}

#[test]
fn point_and_point() {
    let p1 = Point::zeros();
    let p2 = Point::x();
    assert_ne!(p1, p2)
}

#[test]
fn point_at_circle_origin() {
    let p = Point::zeros();
    let c = Circle::new_size(1.0);
    assert!(c.collide_with(&p))
}

#[test]
fn point_in_circle() {
    let p = Point::new(0.9, 0.0);
    let c = Circle::new_size(1.0);
    assert!(c.collide_with(&p))
}

#[test]
fn point_not_in_circle() {
    let p = Point::new(0.9, 0.9);
    let c = Circle::new_size(1.0);
    assert_eq!(c.collide_with(&p), false)
}

#[test]
fn point_in_circle_edge() {
    let p = Point::new(1.0, 0.0);
    let c = Circle::new_size(1.0);
    assert_eq!(c.collide_with(&p), false)
}

#[test]
fn circle_and_itself() {
    let c = Circle::new_size(1.0);
    assert!(c.collide_with(&c))
}

#[test]
fn circle_and_circle_inside() {
    let c1 = Circle::new_size(1.0);
    let c2 = Circle::new_position(0.5, 0.5);
    assert!(c1.collide_with(&c2))
}

#[test]
fn circle_and_circle_seperate() {
    let c1 = Circle::new_size(1.0);
    let c2 = Circle::new_position(1.5, 1.5);
    assert_eq!(c1.collide_with(&c2), false)
}

#[test]
fn circle_and_circle_intersecting() {
    let c1 = Circle::new_size(0.5);
    let c2 = Circle::new_position(0.9, 0.0);
    assert!(c1.collide_with(&c2))
}

#[test]
fn circle_and_circle_edges() {
    let c1 = Circle::new_size(1.0);
    let c2 = Circle::new_position(2.0, 0.0);
    assert_eq!(c1.collide_with(&c2), false)
}
