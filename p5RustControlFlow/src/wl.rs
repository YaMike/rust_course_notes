#![allow(dead_code)]

pub fn while_loop()
{
    let mut x = 1;

    println!("while loop");
    while x < 1000
    {
        x *= 2;

        if x == 64 { continue; }

        println!("x^2 = {}", x);
    }
    println!("just loop");

    let mut y = 1;
    loop // while true
    {
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 { break; }
    }
}

