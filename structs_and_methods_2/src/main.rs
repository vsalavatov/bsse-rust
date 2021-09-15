fn main() {
    use structs_and_methods::geometry::*;
    let p = Point { x: 2.0, y: 2.0 };
    let p2 = Point { x: 20.0, y: 30.0 };
    println!("Point 1: {} {}", p.x, p.y);
    println!("Point 2: {} {}", p2.x, p2.y);
    let rect = Rect {
        xleft: 1.0,
        xright: 3.0,
        ylow: 1.0,
        yhigh: 3.0,
    };
    println!(
        "Rect (ldru): {} {} {} {}",
        rect.xleft, rect.ylow, rect.xright, rect.yhigh
    );
    println!("Rect area: {}", rect.area());
    println!("Rect contains Point 1: {}", rect.contains(&p));
    println!("Rect contains Point 2: {}", rect.contains(&p2));

    let circle = Circle {
        x: 5.0,
        y: 5.0,
        r: 5.0,
    };
    println!("Circle: {} {} {}", circle.x, circle.y, circle.r);
    println!("Circle area: {}", circle.area());
    println!("Circle contains Point 1: {}", circle.contains(&p));
    println!("Circle contains Point 2: {}", circle.contains(&p2));

    let frect = Figure::Rect(rect);
    let fcirc = Figure::Circle(circle);
    println!(
        "Rect as a figure contains P1: {}, P2: {}",
        frect.contains(&p),
        frect.contains(&p2)
    );
    println!(
        "Circle as a figure contains P1: {}, P2: {}",
        fcirc.contains(&p),
        fcirc.contains(&p2)
    );
}
