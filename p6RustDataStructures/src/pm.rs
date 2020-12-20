#![allow(dead_code)]
#![allow(unused_variables)]


fn how_many(x:i32) -> &'static str
{
    // used symbols:
    // | - "or" statement
    // ... - inclusive range
    match x
    {
        0 => "no",
        1 | 2 => "one or two",
        z@ 9...11 => "lots of",
        12 => "a dozen",
        _ if (x % 2 == 0) => "even",
        _ => "a few"
    }
}

pub fn pattern_matching()
{
    println!("\npattern matching examples");

    for x in 0..13
    {
        println!("oranges: {}, {}", x, how_many(x))
    }

    // matching tuple
    let point = (3, 4);

    match (point)
    {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("({},{})", x, y),
        (_, y) => println!("coords: (?,{})", y),
    }
}
