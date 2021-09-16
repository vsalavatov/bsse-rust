pub mod geometry {
    use noisy_float::prelude::*;
    use num_traits::Num;
    use std::{convert::TryFrom, fmt::Debug};

    #[derive(Hash, PartialEq, Eq)]
    pub struct Point<T: Num> {
        pub x: T,
        pub y: T,
    }

    impl<T: Num> Default for Point<T> {
        fn default() -> Self {
            Point {
                x: T::zero(),
                y: T::zero(),
            }
        }
    }

    impl<T: Num + Debug> Debug for Point<T> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            fmt.debug_struct("Point")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }

    #[derive(Hash, PartialEq, Eq)]
    pub struct Rect<T: Num> {
        pub xleft: T,
        pub xright: T,
        pub ylow: T,
        pub yhigh: T,
    }

    impl<T: Num> Default for Rect<T> {
        fn default() -> Self {
            Rect {
                xleft: T::zero(),
                xright: T::one(),
                ylow: T::zero(),
                yhigh: T::one(),
            }
        }
    }

    impl<T: Num + Debug> Debug for Rect<T> {
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
    pub struct Circle<T: Num> {
        pub x: T,
        pub y: T,
        pub r: T,
    }

    impl<T: Num> Default for Circle<T> {
        fn default() -> Self {
            Circle {
                x: T::zero(),
                y: T::zero(),
                r: T::one(),
            }
        }
    }

    impl<T: Num + Debug> Debug for Circle<T> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            fmt.debug_struct("Circle")
                .field("x", &self.x)
                .field("y", &self.y)
                .field("radius", &self.r)
                .finish()
        }
    }

    #[derive(Hash, PartialEq, Eq)]
    pub enum Figure<T: Num> {
        Circle(Circle<T>),
        Rect(Rect<T>),
    }

    // I guess enumeration shouldn't have any default values in this case...

    impl<T: Num + Debug> Debug for Figure<T> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            match &self {
                Figure::Circle(c) => fmt.write_str("Figure::").and_then(|_| c.fmt(fmt)),
                Figure::Rect(r) => fmt.write_str("Figure::").and_then(|_| r.fmt(fmt)),
            }
        }
    }

    impl<T: Num + Ord + Copy> Rect<T> {
        pub fn contains(&self, p: &Point<T>) -> bool {
            self.xleft <= p.x && p.x <= self.xright && self.ylow <= p.y && p.y <= self.yhigh
        }

        pub fn area(&self) -> T {
            (self.xright - self.xleft) * (self.yhigh - self.ylow)
        }
    }

    // Float trait is for .sqrt
    impl<T: Num + Ord + TryFrom<f64> + Float + Copy> Circle<T>
    where
        <T as TryFrom<f64>>::Error: Debug,
    {
        pub fn contains(&self, p: &Point<T>) -> bool {
            let dx: T = p.x - self.x;
            let dy: T = p.y - self.y;
            let dist = (dx * dx + dy * dy).sqrt();
            dist <= self.r
        }

        pub fn area(&self) -> T {
            T::try_from(std::f64::consts::PI).unwrap() * self.r * self.r
        }
    }

    impl<T: Num + Ord + TryFrom<f64> + Float + Copy> Figure<T>
    where
        <T as TryFrom<f64>>::Error: Debug, // thanks to the rust-analyzer for this line!
    {
        pub fn contains(&self, p: &Point<T>) -> bool {
            match self {
                Figure::Circle(c) => c.contains(p),
                Figure::Rect(r) => r.contains(p),
            }
        }
    }
}
