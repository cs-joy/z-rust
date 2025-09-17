fn simple_operation(a: u8, b: u8) {
    println!("Calculator\n");
    println!("value1: {}, value2: {}\n", a, b);
    println!("addition: {}", a+b);
    if a>b {
        println!("subtraction: {}", a-b);
    } else {
        println!("subtraction: {}", b-a);
    }
   if a ==0 || b == 0 {println!("multi: 0")} else {println!("multi: {}", a*b)};
    if b!=0 {println!("div: {}", a/b)} else {println!("error")}
}

fn main() {
    let x: u8 = 16;
    let mut y: u8 = 8;

    // convert 
    let m: u16 = 4;
    y = m as u8;

    simple_operation(x, y);
}

/*
 * Expected Output
 * * * * * *  * * 
Calculator

value1: 16, value2: 4

addition: 20
subtraction: 12
multi: 64
div: 4
*/

