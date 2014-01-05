//A basic rust program looks like this
fn main() {
    println!("Hello!")
}
//Very C like. We can do basic maths
let a = 5 + 4;
let b = a + 2;
//What's this? We have types!
let c = b * 0.1;
//Typecasting is nice
let c = b as f64 * 0.1;
//Nooo, c is not mutable!
c += 1.0

//How about functions?
//Always need type signatures in functions
fn increment (a: uint) -> uint {
    a + 1
}
//If a line isn't ended with a semicolon, it is an expression
//The last line of any function is returned

//As everything's an expression, we get some fun results
let a = if (b > 0) {
    "Positive"
} else {
    "Negative"
}

//We also get pattern matching
match a {
    3 | 5 => "Lol",
    _ => "Not lol"
}
X
//And first class functions
fn munge_tuple((a: uint, b: uint, c: uint), munger: |a: uint| -> uint) -> (uint, uint, uint) {
    (munger(a), munger(b), munger(c))
}

//Give me types! We have structs, which are like their C equivalent
struct a {
    b: uint
}
//Which can then have methods attached, for object oriented programming. But nobody wants that
//Everything's copy by default, and on the stack. We've got pretty fine grained control over memory, and the type system keeps us safe.

//Here's an owned pointer - when it goes out of scope it gets released
let owned = ~a{b: 5};

//We can Borrow pointers from it, as long as the compiler can guarantee its life
let borrowed = &owned

//If we reassign Owned, it gets moved!
let pwned = owned; //Owned can no longer be used

//Magical algerbraic data types
enum MaybeInt {
    Some(int),
    None
}

//And we can unpack them with 
