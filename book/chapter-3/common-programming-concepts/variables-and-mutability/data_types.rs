// Integer type in Rust

fn main() {
    println!("Integer Data types in Rust!\n\n");
    let a: i8 = 92; // range [ -(2^{n-1}) to 2^{n-1}-1 ]
    let b: u8 = 233; // range [ 0 to 2^{n}-1 ]
    let c: i16 = -412;
    let d: u16 = 94;
    let e: i32 = 72;
    let f: u32 = 69;
    let g: i64 = -69;
    let h: u64 = 811;
    let i: i128 = 100101;
    let j: u128 = 111;
    let archdep: isize = 01010101;
    let archdepu: usize = 101001010;

    println!("a:{}\nb:{}\nc:{}\nd:{}\ne:{}\nf:{}\ng:{}\nh:{}\ni:{}\nj:{}\narchdep:{}\narchdepu:{}", a, b, c, d, e, f, g, h, i, j, archdep, archdepu);
}
