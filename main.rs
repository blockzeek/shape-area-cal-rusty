trait Area {
    fn area(self) -> f32;
}

struct Circle {
    radius: f32,
}

impl Area for Circle {
    fn area(self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f32,
    height: f32,
}

impl Area for Triangle {
    fn area(self) -> f32 {
        self.base * self.height / 2f32
    }
}

struct Square {
    side: f32,
}

impl Area for Square {
    fn area(self) -> f32 {
        self.side * self.side
    }
}

fn main() {
    const R: f32 = 1.1f32;
    let circle = Circle { radius: R };
    println!("circle with radius {} has area of {}", R, circle.area());

    const B: f32 = 2.2;
    const H: f32 = 2.2;
    let triangle = Triangle { base: B, height: H };
    println!(
        "triangle with base {} and height {} has area of {}",
        B,
        R,
        triangle.area()
    );

    const S: f32 = 3.3;
    let square = Square { side: S };
    println!("square with side {} has area of {}", S, square.area());
}
