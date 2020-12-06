#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::mem;

enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8},
}

pub fn enums()
{
    println!("\nenumerations example");

    //let c: Color = Color::RgbColor(0,0,0);
    let c: Color = Color::CmykColor{cyan:0, magenta:128, yellow:0, black:255};

    match c
    {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0) => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r, g, b),
        Color::CmykColor{cyan:_, magenta:_, yellow:_, black:255} => println!("black"),
        _ => ()
    }
}
