struct Circle<'r> {
    radius: &'r f32,
}

impl<'a> Circle<'a> {
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

fn main() {
    let radius = 7.0f32;
    let circle = {
        Circle {
            radius: &radius,
        }
    };

    println!("Area: {}", circle.area());
}
