fn main() {
//---START:1-1
    let a = 5 + 4;
    let b = a + 2;
//---END:1-1
//---START:1-2
    let c: int = 7;
//---END:1-2
//---START:2-1
    let a = 2;
    /*
    let b = a * 0.1; //Won't compile
    */
//---END:2-1
//---START:2-2
    let b = a as f64 * 0.1;
//---END:2-2
//---START:3-1
    let a = 1.0;
    /*
    a += 1.0 // Won't compile
    */
//---END:3-1
//---START:3-2
    let mut a = 5;
    a += 1;
//---END:3-2
//---START:3-3
    let a = 1;
    let a = 5;
//---END:3-3
}
