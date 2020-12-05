#![allow(dead_code)]

pub fn if_statement()
{
    let temp = 33;
    let upper_boundary = 30;
    let lower_boundary = 10;

    // regular if usage
    if temp > upper_boundary
    {
        println!("temp {} is greater than {}", temp, upper_boundary);
    }
    else if temp < lower_boundary
    {
        println!("temp {} is less than {}", temp, lower_boundary);
    }
    else
    {
        println!("temp {} is between {} and {}", temp, upper_boundary, lower_boundary);
    }

    // if can be used as expression
    let day = if temp > 30 {"hot"} else {"warm"};
    println!("today is {} day", day);

    println!("is it {}",
        if temp > 30 {"hot"} else if temp < 10 {"cold"} else {"good"});

    println!("it is {}",
        if temp > 20 {
            if temp > 30 {"too hot"} else {"just hot"}
        } else if temp < 10 {"cold"} else {"OK"});
}
