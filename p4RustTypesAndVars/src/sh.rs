#![allow(dead_code)]
use std::mem;

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{
    println!("\nstack_and_heap");
    let p1 = origin();
    let p2 = Box::new(origin());

    // no overhead for stack var - 16 bytes
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    // takes 8 bytes, for 64-bit OS, since its the pointer
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    // Boxed value back to the stack
    let p3 = *p2; 
    println!("p3.x is {}", p3.x);
}
