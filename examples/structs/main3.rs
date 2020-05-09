struct Circle<'r> {
    radius: &'r f32,
}

impl<'r> Circle<'r> {
    fn get_radius(&self) -> &'r f32 {
        self.radius
    }
}

fn main() {
    let radius = 7.0f32;
    let borrow = {
        let circle = Circle {
            radius: &radius,
        };

        circle.get_radius()
    };

    println!("Borrow: {}", borrow);
}
