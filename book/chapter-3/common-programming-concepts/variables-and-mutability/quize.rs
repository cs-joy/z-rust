fn main() {
    let mut x: u32 = 1;
    {
        let mut x = x;
        x += 2;
    }
    println!("{x}");
}

// expected output: 1
