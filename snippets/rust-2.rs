fn main() {
//---START
    //We can do basic maths
    let a = 5 + 4;
    let b = a + 2;
    // Types are inferred, but you can always provide your own
    let c: int = 7;
    // Types aren't automagically cast
    /*
    let d = c * 0.1; //Won't compile
    */
    //We can manually cast
    let d = b as f64 * 0.1;

    //C is not mutable, so the following won't work
    /*
    c += 1.0 // Won't compile
    */

    //When defining multiple lets with the same variable name,
    //The earlier definition gets hidden
    let d = 5;

    //If we want mutability, we have to specify it.
    let mut d = 5;
//---END
}
