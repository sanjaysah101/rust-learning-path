use std::f64::consts::PI;

trait Measurable {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Measurable for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }
}

impl Measurable for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}

impl Measurable for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn name(&self) -> &str {
        "Triangle"
    }
}

fn main() {
    let circle: Box<dyn Measurable> = Box::new(Circle { radius: 2.0 });
    let rectangle: Box<dyn Measurable> = Box::new(Rectangle {
        height: 4.0,
        width: 5.0,
    });
    let triangle: Box<dyn Measurable> = Box::new(Triangle {
        base: 4.0,
        height: 5.0,
    });

    let mut shapes = Vec::<Box<dyn Measurable>>::new();
    shapes.push(circle);
    shapes.push(rectangle);
    shapes.push(triangle);

    for shape in shapes {
        println!("The area of {} is {}", shape.name(), shape.area())
    }
}
