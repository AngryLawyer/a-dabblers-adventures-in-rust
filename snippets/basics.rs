fn main() {
//---START:1
    //We can do basic maths
    let a = 5 + 4;
    let b = a + 2;
    // Types are inferred, but you can always provide your own
    let c: int = 7;
//---END:1
//---START:2
    let a = 2;
    // Types aren't automagically cast
    /*
    let b = a * 0.1; //Won't compile
    */
    //We can manually cast
    let b = a as f64 * 0.1;
//---END:2
//---START:3
    let a = 1.0;
    //a is not mutable, so the following won't work
    /*
    a += 1.0 // Won't compile
    */

    //If we want mutability, we have to specify it.
    let mut d = 5;

    //When defining multiple lets with the same variable name,
    //The earlier definition gets hidden
    let a = 5;

//---END:3
}
