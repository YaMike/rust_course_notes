#[allow(dead_code)]
#[allow(unused_variables)]

mod cf;
mod wl;
mod fl;
mod ms;

fn main() {
    println!("if statement");
    cf::if_statement();
    println!("\nwhile loop");
    wl::while_loop();
    println!("\nfor loop");
    fl::for_loop();
    println!("\nmatch statement");
    ms::match_statement();
}
