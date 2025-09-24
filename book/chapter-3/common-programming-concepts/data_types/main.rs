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
use std::io;


fn data_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let guess:u8 = "42".parse().expect("NaN");
    data_type(&guess);
    println!("{guess}");

    // integers data types
    // signed
    // range: [-(2^{n-1}) : (2^{n-1}-1)]
    println!("Integer (Signed)");
    
    let num = -24;
    data_type(&num); // default data type(integer): i32

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
    
    // integer (unsigned)
    // range: [0:2^n-1]
    println!("\nInteger (Unsigned)");
    
    let num = 0;
    data_type(&num); // default data type(integer): i32

    let x:u8 = 26;
    data_type(&x);

    let x:u16 = 256;
    data_type(&x);

    let x:u32 = 678;
    data_type(&x);

    let x:u64 = 875094;
    data_type(&x);

    let x:u128 = 7458978943;
    data_type(&x);
/**
Number literals	Example (Integers)
Decimal:	98_222
Hex:	0xff
Octal:	0o77
Binary:	0b1111_0000
Byte (u8 only):	b'A'
**/
    
    // floating-point types
    // 2 types: f32 and f64(by default)
    println!("\n\nFloating-Point");
    
    let m = 65.3481; // by default f64
    data_type(&m);
    
    let n:f32 = 23.49604;
    data_type(&n);
    
    // boolean types
    println!("\n\nBoolean");
    
    let isCall = true;
    data_type(&isCall);
    
    let isPut: bool = false;
    data_type(&isPut);
    
    // character types
    println!("\n\nCharacter");
    
    let myChar = 'A';
    data_type(&myChar);
    
    let z: char = 'ℤ'; // with explicit type annotation
    data_type(&z);
    
    let emoji = '😻';
    data_type(&emoji);
    
    
    
    // Compound Types
    // two primitive types
    // 1. tuples 	2. arrays
    
    // Tuples
    // fixed length. once declared, they can't grow or shrink in size
    println!("\n\nTuples");
    
    let mut tup: (i32, f64, bool, char) = (32, 52.21, true, 'Z',); // initialize tuples with given data types
    data_type(&tup);
    
    // other way to initialize tuples
    let tupl = (88.4, 'Z', 0, -55);
    data_type(&tupl);
    
    // accessing value of the tuples using `.` and indices. for instance `tup.0` to access first element of the tuples
    let access_first_element = tup.0;
    print!("tup.0: {} and data_type: ", access_first_element);
    data_type(&access_first_element);
    
    let access_second_element = tup.1;
    print!("tup.1: {} and data_type: ", access_second_element);
    data_type(&access_second_element);
    
    let access_third_element = tup.2;
    print!("tup.2: {} and data_type: ", access_third_element);
    data_type(&access_third_element);
    
    let access_fourth_element = tup.3;
    print!("tup.3: {} and data_type: ", access_fourth_element);
    data_type(&access_fourth_element);
    
    // modifying the element (not data type, you can't do that)
    // to modifying the element of the tuples, as we know we have to use `mut` for enabling mutability
    // so, now i just add `mut` keyword in the `tup` variable
    
    // updating the value of first element and so on
    tup.0 += 2; // so, tup.0 become increase by 2 so 34
    tup.1 -= 0.21; // tup.1 become decrease by 0.21 so 52.0
    tup.2 = false; // tup.2 become false
    tup.3 = 'S'; // tup.3 become 'S'
    
    // now check if updated
    println!("tup.0= {}", tup.0);
    println!("tup.1= {}", tup.1);
    println!("tup.2= {}", tup.2);
    println!("tup.3= {}", tup.3);
    
    // define tuples without give any element
    //let tip: (i8, f32, bool) = (); // return an ERROR
    let tip = ();
    data_type(&tip); // unit type,, return `()`
    
    
    // Array Types
    // Unlike a tuple, every element of an array must have the same type. fixed length 
    println!("\n\nArray");
    
    let mut arr = [2, 4, 6, 8];
    data_type(&arr); // return [i32; 4], [data_type and number of element in the arr]
    
    // arrays are more useful when you know the number of elements will not need to change.
    // for instance:
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    data_type(&months); // return [&str; 12]
    println!("months[0]: {}", months[0]);
    
    let vowel = ['a', 'e', 'i', 'o', 'u'];
    data_type(&vowel); // return [char; 5]
    println!("vowel[2]: {}", vowel[2]); // accessing element of the array using `[]` square bracket
    
    // modifying elements of the arr
    // every elements is incremented by 1
    arr[0] += 1;
    arr[1] += 1;
    arr[2] += 1;
    arr[3] += 1;
    
    // print to check if updated
    println!("arr[0]: {}", arr[0]);
    println!("arr[1]: {}", arr[1]);
    println!("arr[2]: {}", arr[2]);
    println!("arr[3]: {}", arr[3]);
    
    // initialize array with given type
    let brr: [f32; 3] = [0.0425, 0.01, 0.25];
    data_type(&brr);
    
    // define array without give any element int the array
    //let data: [f64; 35] = []; // return an ERROR
    //let data = []; // also return an ERROR
    
    // we can also initialize an array to contain the same value for each element
    // by::::
    let a = [8; 5]; // [specifying_value_of_the_array; number_of_value]
    data_type(&a);
    print!("[{}, ", a[0]);
    print!("{}, ", a[1]);
    print!("{}, ", a[2]);
    print!("{}, ", a[3]);
    println!("{}]", a[4]);
    
    // Arrays are useful when you want your data allocated on the `stack`, the 
    // same as the other types we have seen so far, rather than the `heap` or,
    // when you want to ensure you always have a fixed number of elements. 
    // An array isn’t as flexible as the vector type, though.
    // A vector is a similar collection type provided by the `standard library` that is allowed
    // to grow or shrink in size because its contents live on the heap.

    
    // Invalid array element access
    fn invalid_array_element_access() {
    	let a = [1, 2, 3, 4, 5];

    	println!("Please enter an array index.");

    	let mut index = String::new();

    	io::stdin()
           .read_line(&mut index)
           .expect("Failed to read line");

    	let index: usize = index
           .trim()
           .parse()
           .expect("Index entered was not a number");

    	let element = a[index];

    	println!("The value of the element at index {index} is: {element}");
    }
    invalid_array_element_access();
    // NOTE
    // This code compiles successfully. If you run this code using cargo run and enter 0, 1, 2, 3, or 4, the program will print out the corresponding value at that index in the array. If you instead enter a number past the end of the array, such as 6, you’ll see output like this:
/*
thread 'main' panicked at main.rs:265:23:
index out of bounds: the len is 5 but the index is 6
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
/*
The program resulted in a runtime error at the point of using an invalid value in the indexing operation. The program exited with an error message and didn’t execute the final println! statement. When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the length, Rust will panic. This check has to happen at runtime, especially in this case, because the compiler can’t possibly know what value a user will enter when they run the code later.

This is an example of Rust’s memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing. 
*/


    // quize
    fn quize() {
 	 let tup_arr = ([1; 2], [3; 3], [3.5; 4], [true; 2], ['Z';1]);
 	 data_type(&tup_arr);
 	 let (a, b, c, d, e) = tup_arr;
 	 print!("data type of the third element in the tuple: ");
 	 data_type(&c);

  	// accessing value of the first element of `a`
 	 println!("a[0]: {}", a[0]);

  	// another way to access
  	println!("{}", tup_arr.4[0]);

  	// operation
 	 println!("sum = {}", a[0] + tup_arr.1[0]); // return 1 + 3 becomes 4
    }
    quize();
    
    // source: https://rust-book.cs.brown.edu/ch03-02-data-types.html
}

