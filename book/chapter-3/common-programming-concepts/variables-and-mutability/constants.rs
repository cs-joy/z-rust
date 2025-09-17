/**
 * @source - https://rust-book.cs.brown.edu/ch03-01-variables-and-mutability.html
 * Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.
 *First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable. You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated. We’ll cover types and type annotations in the next section, “Data Types”, so don’t worry about the details right now. Just know that you must always annotate the type.
 * Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
 */
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE HOURS IN SECONDS= {}", THREE_HOURS_IN_SECONDS);
}
