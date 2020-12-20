#![allow(dead_code)]
#![allow(unused_variables)]

struct Point
{
    x: f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

fn structures_simple()
{
    println!("structures example");

    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    (x + y, x * y)
}

fn tuples_simple()
{
    println!("\ntuples example");
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("print the tuple at once using debug output: sp = {:?}", sp);
    println!("tuple by parts: {0} + {1} = {2}, {0} + {1} = {3}", x, y, sp.0, sp.1);

    // desctructuring the tuple - allow using named vars instead of tuple and indexing
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    // constucting tuples of tuples
    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    // printint the last element of tuples of tuples - need round brackets
    println!("last elem = {}", (combined.1).1);

    // desctructuring the tuples of tuples
    let ((c,d),(e,f)) = combined;
    println!("c = {}, d = {}, e = {}, f = {}", c, d, e, f);

    // tuple with a mixed types
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // single element tuple
    let vol = (42);
    println!("{:?}", vol);
}

pub fn structures()
{
    structures_simple();
    tuples_simple();
}

