#![allow(dead_code, unused_variables)]

struct Point<T> {
    x: T,
    y: T,
}

// Implement a generic method
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implement a method for specific types in a generic
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // Functions related to Point1
    let integer = Point { x: 5, y: 10 }; // i32
    let float = Point { x: 1.0, y: 4.0 }; // f64

    println!("integer.x = {}", integer.x()); // 5
    println!(
        "float.distance_from_origin = {}",
        float.distance_from_origin()
    ); // 4.123…

    // Functions related to Point2
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
