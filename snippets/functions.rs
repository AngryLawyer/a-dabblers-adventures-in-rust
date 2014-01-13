fn main() {
//---START:1
    //Functions must specify the types they use
    fn increment (a: uint) -> uint {
        a + 1
    };

    //Functions are first class. Return the last value in them
    fn munge_triple((a, b, c): (uint, uint, uint), 
        munger: |a: uint| -> uint) -> (uint, uint, uint) {
        (munger(a), munger(b), munger(c))
    };
    let a = 1;
    let b = increment(a);

    let c = munge_triple((1, 2, 3), increment);
    //There's also shorthand
    let c = munge_triple((1, 2, 3), |a| a+1);
//---END:1
//---START:2
    //That function's a bit naff, it only takes uints.
    //If only there was another way...
    fn scrunge_triple<T, U>((a, b, c): (T, T, T),
        munger: |a: T| -> U) -> (U, U, U) {
       (munger(a), munger(b), munger(c))
    };

    //Now, the compuler will work out what types the function wants

    let a = scrunge_triple((1, 2, 3), |a| a + 1);
    //Success, because T and U are both substituted with uint

    let b = scrunge_triple((1, 2, 3), |a| a as f64 * 0.1);
    //Success, because T is uint, and U is f64

    //let c = munge_triple(1, 2.0, 3), |a| a * 0.1);

    //Fails the typecheck as all the tuple elements
    //were marked as the same
//---END:2
}
