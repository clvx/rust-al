use std::ops::Add;

struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Point;

    // add takes self and rhs as arguments and returns a Point
    //self is the left-hand side of the + operator
    //rhs is the right-hand side of the + operator
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,  // Add the x values of self and rhs
            y: self.y + rhs.y  // Add the y values of self and rhs
        }
    }
}

fn main() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 3.0, y: 4.0 };
    let c = a + b;
    println!("c.x = {}, c.y = {}", c.x, c.y);
}
