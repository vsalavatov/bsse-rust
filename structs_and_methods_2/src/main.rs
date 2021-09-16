use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn main() {
    use noisy_float::prelude::*;
    use structs_and_methods::geometry::*;

    let p = Point {
        x: n64(2.0),
        y: n64(2.0),
    };
    let p2 = Point {
        x: n64(20.0),
        y: n64(30.0),
    };
    println!("Point 1: {:?}", p);
    println!("Point 2: {:?}", p2);
    let rect = Rect {
        xleft: n64(1.0),
        xright: n64(3.0),
        ylow: n64(1.0),
        yhigh: n64(3.0),
    };
    println!("Rect: {:?}", rect);
    println!("Rect area: {}", rect.area());
    println!("Rect contains Point 1: {}", rect.contains(&p));
    println!("Rect contains Point 2: {}", rect.contains(&p2));

    let circle = Circle {
        x: n64(5.0),
        y: n64(5.0),
        r: n64(5.0),
    };
    println!("Circle: {:?}", circle);
    println!("Circle area: {}", circle.area());
    println!("Circle contains Point 1: {}", circle.contains(&p));
    println!("Circle contains Point 2: {}", circle.contains(&p2));

    let frect = Figure::Rect(rect);
    let fcirc = Figure::Circle(circle);
    println!("Rect as a figure: {:?}", frect);
    println!(
        "Rect as a figure contains P1: {}, P2: {}",
        frect.contains(&p),
        frect.contains(&p2)
    );
    println!("Circle as a figure: {:?}", fcirc);
    println!(
        "Circle as a figure contains P1: {}, P2: {}",
        fcirc.contains(&p),
        fcirc.contains(&p2)
    );

    let rect = Rect {
        xleft: n64(1.0),
        xright: n64(3.0),
        ylow: n64(1.0),
        yhigh: n64(3.0),
    };
    let mut hasher = DefaultHasher::new();
    &rect.hash(&mut hasher);
    println!("Rect hash is {:x}!", hasher.finish());

    let irect: Rect<i8> = Rect::default();
    println!("i8 rect: {:?}, area: {:?}", irect, irect.area());
    let icirc: Circle<i32> = Circle { x: 3, y: 4, r: 5 };
    println!("i32 circle: {:?}", icirc);
    // println!("i32 circle area: {}", icirc.area()); // method cannot be called on `structs_and_methods::geometry::Circle<i32>` due to unsatisfied trait bounds
}
