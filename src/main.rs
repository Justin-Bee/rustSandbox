/*
 Learning Rust
 */

//include the print file and its functions
mod print;

fn main() {

    //use the print function in the print file
    print::print();

    let mut x = 10;  //mut keyword makes the variale mutable, needs if variable needs to change
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);

    //specify type
    let y: u8 = 255;
    //y += 1; //causes a panic, due to overflow
    println!("y is {}", y);

    //RUST supports 2 floating pt types f32 and f64
    let z = 10.122324546577976324354;
    println!("Z is {}", z);

    //casting types
    let a = 10;
    let b = 3.0;
    let c = a as f64 / b; //cast a as 64 bit float
    println!("C is {}", c);
}
