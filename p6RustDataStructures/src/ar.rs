#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;


fn simple_array()
{
    let mut a:[i32;5] = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 777;

    println!("a has {} elements, first is {}", a.len(), a[0]);

    // debug output for printint the whole content of the array
    println!("{:?}", a);

    // comparing the array
    if a != [1, 2, 3, 4, 5]
    {
        println!("array a doesn't match array [1,2,3,4,5]")
    }
    else if a == [777, 2, 3, 4, 5]
    {
        println!("however, array a is equal to [777,2,3,4,5]")
    }

    // filling the whole array -  bulk fill
    let b = [1u16;10]; // ten elements all equal to 1 (type of unsigned 16-bit)

    for i in 0..b.len()
    {
        println!("b[{}]={}", i, b[i])
    }

    // how much memory array occupies
    println!("b (len - {}) took up {} bytes", b.len(), mem::size_of_val(&b));


    let mtx:[[f32;3];2] = 
    [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0]
    ];

    println!("debug print for mtx: {:?}", mtx);


    println!("main diagonal:");
    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i == j
            {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j])
            }
        }
    }
}

fn use_slice(slice: &mut[i32])
{
    let a = 777;
    println!("first element {}, len={}", slice[0], slice.len());
    println!("change the second element from {} to {}", slice[0], a);
    slice[2] = a;
    for i in 0..slice.len()
    {
        println!("slice[{}] = {}", i, slice[i])
    }
}


fn slices()
{
    println!("\nslices example");

    let mut data = [1, 2, 3, 4, 5];

    // pass from second up to fourth element - three elements in total:
    use_slice(&mut data[1..4]);
    // pass the whole array: use_slice(&mut data);
    use_slice(&mut data);
}

pub fn arrays()
{
    println!("\narrays example");
    simple_array();
    slices();
}
