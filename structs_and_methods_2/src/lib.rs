pub mod geometry {
    use std::fmt::{Debug, Write};

    use noisy_float::prelude::*;

    #[derive(Hash, PartialEq, Eq)]
    pub struct Point {
        pub x: N64,
        pub y: N64,
    }

    impl Default for Point {
        fn default() -> Self {
            Point {
                x: n64(0.0),
                y: n64(0.0),
            }
        }
    }

    impl Debug for Point {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            fmt.debug_struct("Point")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }

    #[derive(Hash, PartialEq, Eq)]
    pub struct Rect {
        pub xleft: N64,
        pub xright: N64,
        pub ylow: N64,
        pub yhigh: N64,
    }

    impl Default for Rect {
        fn default() -> Self {
            Rect {
                xleft: n64(0.0),
                xright: n64(1.0),
                ylow: n64(0.0),
                yhigh: n64(1.0),
            }
        }
    }

    impl Debug for Rect {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            fmt.debug_struct("Rectangle")
                .field("x_left", &self.xleft)
                .field("y_low", &self.ylow)
                .field("x_right", &self.xright)
                .field("y_high", &self.yhigh)
                .finish()
        }
    }

    #[derive(Hash, PartialEq, Eq)]
    pub struct Circle {
        pub x: N64,
        pub y: N64,
        pub r: N64,
    }

    impl Default for Circle {
        fn default() -> Self {
            Circle {
                x: n64(0.0),
                y: n64(0.0),
                r: n64(1.0),
            }
        }
    }

    impl Debug for Circle {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            fmt.debug_struct("Circle")
                .field("x", &self.x)
                .field("y", &self.y)
                .field("radius", &self.r)
                .finish()
        }
    }

    #[derive(Hash, PartialEq, Eq)]
    pub enum Figure {
        Circle(Circle),
        Rect(Rect),
    }

    // I guess enumeration shouldn't have any default values in this case...

    impl Debug for Figure {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            match &self {
                Figure::Circle(c) => fmt.write_str("Figure::").and_then(|_| c.fmt(fmt)),
                Figure::Rect(r) => fmt.write_str("Figure::").and_then(|_| r.fmt(fmt)),
            }
        }
    }

    impl Rect {
        pub fn contains(&self, p: &Point) -> bool {
            return self.xleft <= p.x
                && p.x <= self.xright
                && self.ylow <= p.y
                && p.y <= self.yhigh;
        }

        pub fn area(&self) -> N64 {
            return (self.xright - self.xleft) * (self.yhigh - self.ylow);
        }
    }

    impl Circle {
        pub fn contains(&self, p: &Point) -> bool {
            let dx: N64 = p.x - self.x;
            let dy: N64 = p.y - self.y;
            let dist = (dx * dx + dy * dy).sqrt();
            return dist <= self.r;
        }

        pub fn area(&self) -> N64 {
            return n64(std::f64::consts::PI) * self.r * self.r;
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
