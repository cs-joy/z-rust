/** @source: https://rust-book.cs.brown.edu/ch03-01-variables-and-mutability.html
 *
 * Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.
 *
 * The other difference between `mut` and `shadowing` is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
 * for instance: 
 *              say our program asks a user to show how many spaces they want between some text by
 *              inputting space characters, and then we want to store that input as a number: check
 *              `second_diff()` function.
 */
fn data_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn second_diff() {
    let spaces = "    "; // 4 space
    data_type(&spaces); // string type
    let spaces = spaces.len();
    data_type(&spaces); // number type
                        //
                        //
    // if we try to use `mut` for this, as shown here, we’ll get a compile-time error:
    //let mut spaces = "    ";
    //spaces = spaces.len();
    //
    //to see the compile-time error, please uncomment the above code. :)
}


fn main() {
    println!("First difference between mut and shadowing\n");
    let discount = 0.2;
    let bill = 100.35;
    let employee_card_type = "CREDIT";
    if employee_card_type == "CREDIT" {
        let discount = discount + 0.3; // shadowing of variable
        let bill = bill - (bill*discount); // shadowing of second variable
        println!("you have to pay[CREDIT]: ${}", bill);
    }
    let bill = bill - (bill*discount);
    println!("you have to pay[DEBIT]: ${}", bill);

    println!("\nSecond difference:\n");
    second_diff();
}
