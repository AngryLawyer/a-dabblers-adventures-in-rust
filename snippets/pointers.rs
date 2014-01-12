fn main () {
//---START
    struct Llama {
        hairiness: uint
    };

    fn new_llama(hairiness: uint) -> Llama {
        Llama{hairiness: hairiness}
    };

    //Anything unadorned is allocated on the stack. It is freed when it falls out of scope
    let hose = new_llama(2);

    //We can define Owned pointers - allocated on the heap, and freed when the pointer goes out of scope
    let ferdinand = ~new_llama(3);
    //They're guaranteed unique - you can move them
    let another_llama = ferdinand;
    //The following won't compile
    //let fluff = ferdinand.hairiness;

    //You can borrow pointers as long as they expire before the owned pointer
    let borrowed_llama = &another_llama;
    //The compiler keeps check and will shout at you if you get it wrong.
//---END
}
