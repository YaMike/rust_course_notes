#![allow(dead_code)]
#![allow(unused_variables)]

// 32-bit example
union IntOrFloat
{
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat)
{
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value")
            }

            IntOrFloat { f } => {
                println!("value = {}", f)
            }
        }
    }
}

pub fn unions()
{
    println!("\nunions example");
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("integer value is {}", value);

    process_value(IntOrFloat{f:42.0});
    // aka "reinterpret case" - integer value is considered as a floating point
    process_value(IntOrFloat{i:5});
}
