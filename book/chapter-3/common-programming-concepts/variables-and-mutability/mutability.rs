fn main() {
    let x: i16 = -876; // immutable
    //x = -10;          // it causes error because `x` is not changeable
    println!("x= {}", x);
    let mut y: u16 = 876;  // mutable
    y += 4;
    println!("y= {}", y); // output: y = 880
}
