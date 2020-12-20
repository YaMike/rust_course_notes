#![allow(dead_code)]
#![allow(unused_variables)]


fn simple()
{
    let x = 3.14;
    //let y = 0.0;
    let y = 2.71828;

    // Option -> Some(v) | None
    let result =
        if y != 0.0 { Some(x / y)} else { None };

    // using match for the result
    match result {
        Some(z) => println!("{} / {} = {}", x, y, z),
        None => println!("canont devide by zero")
    }

    // using in if-statement - no handle for negative case
    if let Some(z) = result {
        println!("result = {}", z)
    }

    // alternativ - using while:
    // while let Some(z) 
}

pub fn options()
{
    println!("\ntemplates examples");
    simple();
}
