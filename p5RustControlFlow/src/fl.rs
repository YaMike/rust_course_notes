#![allow(dead_code)]

pub fn for_loop()
{
    println!("simple range example");
    for x in 1..11
    {
        if x == 3 { continue; }
        if x == 8 { break; }
        println!("x = {}", x);
    }

    println!("range with enumeration");
    for (pos,y) in (30..41).enumerate()
    {
        println!("{}:{}", pos, y);
    }
}

