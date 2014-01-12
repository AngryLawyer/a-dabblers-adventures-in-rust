//---START
//Functions must specify the types they use
fn increment (a: uint) -> uint {
    a + 1
}

//Functions are first class. Return the last value in them
fn munge_triple((a, b, c): (uint, uint, uint), munger: |a: uint| -> uint) -> (uint, uint, uint) {
    (munger(a), munger(b), munger(c))
}

fn main() {
    let a = 1;
    let b = increment(a);

    let c = munge_triple((1, 2, 3), increment);
    //There's also shorthand
    let c = munge_triple((1, 2, 3), |a| a+1);

}
//---END
