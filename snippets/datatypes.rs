fn main() {
//---START
    //We have structs, which are like their C equivalent.
    struct Llama {
        hairiness: uint
    };

    let jose = Llama{hairiness: 5};

    //We also have tuples
    let tuple = (5, "lol", 7);

    //And a typed list-like structure called vectors
    let my_list = ["first", "second", "third"];

    //Finally, we have enums, which we'll take a closer look at later
    enum Colors {
        Red,
        Blue,
        Green
    };

    let myColor = Red;
//---END
}
