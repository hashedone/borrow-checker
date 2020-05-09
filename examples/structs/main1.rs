struct Circle<'r> {
    radius: &'r f32,
}

impl<'a> Circle<'a> {
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

fn main() {
    let circle = {
        let radius = 7.0f32;
        Circle {
            radius: &radius,
        }
    };

    println!("Area: {}", circle.area());
}
