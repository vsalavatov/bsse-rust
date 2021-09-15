pub mod geometry {
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    pub struct Rect {
        pub xleft: f64,
        pub xright: f64,
        pub ylow: f64,
        pub yhigh: f64,
    }

    pub struct Circle {
        pub x: f64,
        pub y: f64,
        pub r: f64,
    }

    pub enum Figure {
        Circle(Circle),
        Rect(Rect),
    }

    impl Rect {
        pub fn contains(&self, p: &Point) -> bool {
            return self.xleft <= p.x
                && p.x <= self.xright
                && self.ylow <= p.y
                && p.y <= self.yhigh;
        }

        pub fn area(&self) -> f64 {
            return (self.xright - self.xleft) * (self.yhigh - self.ylow);
        }
    }

    impl Circle {
        pub fn contains(&self, p: &Point) -> bool {
            let dx: f64 = p.x - self.x;
            let dy: f64 = p.y - self.y;
            let dist = (dx * dx + dy * dy).sqrt();
            return dist <= self.r;
        }

        pub fn area(&self) -> f64 {
            return std::f64::consts::PI * self.r * self.r;
        }
    }

    impl Figure {
        pub fn contains(&self, p: &Point) -> bool {
            match self {
                Figure::Circle(c) => c.contains(p),
                Figure::Rect(r) => r.contains(p),
            }
        }
    }
}
