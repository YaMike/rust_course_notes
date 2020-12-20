#![allow(dead_code)]
#![allow(unused_imports)]

struct Point<T>
{
    x: T,
    y: T
}

struct Line<T>
{
    start: Point<T>,
    end: Point<T>
}

pub fn generics()
{
    println!("\ngenerics examples");

    let a:Point<f64> = Point { x: 0.0, y: 0f64 };
    let b = Point { x: 1.2, y: 3.4 };

    let myline = Line { start: a, end: b };

    println!("Line: start: ({},{}), end: ({},{})", myline.start.x, myline.start.y, myline.end.x, myline.end.y);
}
