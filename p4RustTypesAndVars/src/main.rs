#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

mod sh;

const SOME_GLOBAL_VAR_EXAMPLE:u8 = 42; // no fixed address
static mut Z:i32 = 1234; // static var


fn operators()
{
    println!("\noperators");
    // arithmetic
    let mut a = 2 + 3 * 4; // +-*
    println!("a = {}", a);
    a = a + 1;
    println!("a = a + 1 => {}", a);
    a += 2; // -= += *= /= %=
    println!("a = a - 2 => {}", a);
    let a_remainder = a % 2;
    println!("remainder of a {} and 2 is {}", a, a_remainder);

    // power for integer numbers
    let a_cubed = i32::pow(a, 3);
    println!("a {} cubed = {}", a, a_cubed);
    
    // power for floating point numbers
    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^PI = {}", b, b_cubed, b, b_to_pi);

    // bitwise operators
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
    println!("1|2 = {}", c);

    // shift operators
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!("pi less 4 = {}", pi_less_4);
    let x = 5;
    let x_is_5 = 5 == x;
    println!("x {} == 5 is {}", x, x_is_5); 
}


fn fundamental_data_types()
{
    println!("\nfundamental_data_types");
    let a: u8 = 123; // u = unsigned, 8 bits, 0-255
    println!("value of a is {}", a);

    let mut b: i8 = -1; // signed, 8 bits, -128..0..127
    println!("value of b is {}", b);
    b += 43;
    println!("value of b is {}", b);

    let mut c = 1234567890; // type guessed by compiler -> i32
    println!("value of c is {}, size of c is {} bytes",
             c, mem::size_of_val(&c));

    c /= 2;
    println!("value of c is {}, size of c is {} bytes",
             c, mem::size_of_val(&c));

    let z: isize = 123; // using OS native signed types
    let size_of_z = mem::size_of_val(&z);
    println!("value of z is {}, size of z is {} bytes ({}-bit OS)",
             z, size_of_z, 8 * size_of_z);

    let d: char = 'x'; // use char
    println!("value of d is {}, size of d is {} bytes",
             d, mem::size_of_val(&d));
    
    let e: f32 = 2.5; // use floating number of 32-bit size
    println!("value of e is {}, size of e is {} bytes",
             e, mem::size_of_val(&e));

    let g: bool = false; // use boolean values
    println!("value of g is {}, size of g is {} bytes",
             g, mem::size_of_val(&g));
}


fn scope_and_shadowing()
{
    println!("\nscope_and_shadowing");
    let a = 123;

    {
        let b = 456;
        println!("b = {}", b);
        let a = 789;
        println!("inner scope a = {}", a);
    }

    println!("outer scope a = {}", a);

    println!("SOME_GLOBAL_VAR_EXAMPLE = {}", SOME_GLOBAL_VAR_EXAMPLE);

    // need to mark block unsafe in case static mutable use
    unsafe
    {
        println!("static global var Z = {}", Z);
    }
}


fn main() {
    operators();
    fundamental_data_types();
    scope_and_shadowing();
    sh::stack_and_heap();
}
