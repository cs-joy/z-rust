// data types:
//                      1. Scalar
//                            #Primary types
//                                 1.1> Integer types
//                                 1.2> Floating-point 
//                                 1.3> The Boolean
//                                 1.4> The Character
//                      2. Compound
//                           #Primitive types are
//                                 2.1> The Tuple
//                                 2.2> The Array

/*
# Integer Types

Prerequisite:
about bit
1 bit > 1 or 0
8 bit > 11011001
1 byte > 8 bit and can store 2^N number of different values where N is the number of bit, hence
8 bit can store, 2^8=256 different values
!!!!!!!!!!!!!!!!!

built-in integer type in #Rust
#Signed
1. i8(length: 8 bit)(range: -128:127)
2. i16(length: 16bit)(range: -32768:32767)
3. i32(length: 32bit)
4. i64(length: 64bit)
5. i128(length: 128bit)

#Unsigned (length are same)
1. u8
2. u16
3. u32
4. u64
5. u128
 */

fn data_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let guess:u8 = "42".parse().expect("NaN");
    data_type(&guess);
    println!("{guess}");

    // integers data types
    let num = 24;
    data_type(&num); // default data type(integer): u32

    let x:i8 = 26;
    data_type(&x);

    let x:i16 = 256;
    data_type(&x);

    let x:i32 = 678;
    data_type(&x);

    let x:i64 = 875094;
    data_type(&x);

    let x:i128 = 7458978943;
    data_type(&x);
    // 
}

