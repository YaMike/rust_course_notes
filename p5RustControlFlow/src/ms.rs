#![allow(dead_code)]

pub fn match_statement()
{
    let country_code = 44;
    let country = match country_code {
        44 => "UK",
        46 => "SW",
        7  => "RU",
        1..=1000 => "Unknown",
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country);

    let x = false;
    let s = match x {
        true => "true",
        false => "false"
    };

    println!("x {} is s {}", x, s);
}
