fn main () {
//---START:1
    struct Kitty {
        fluffiness: uint
    };

    fn new_kitty(fluffiness: uint) -> Kitty {
        Kitty{fluffiness: fluffiness}
    };

    let buffcat = new_kitty(2); //On the stack
//---END:1
//---START:2-1
    let tom = ~new_kitty(3);
    //They're guaranteed unique - you can move them
    let moved_tom = tom;
    //The following won't compile, as it no longer lives in that slot
    //let fluff = tom.fluffiness;
//---END:2-1
//---START:2-2
    let borrowed_kitty = &buffcat;
    //We can still use the original
    let is_the_same = borrowed_kitty.fluffiness == buffcat.fluffiness;
//---END:2-2
}
