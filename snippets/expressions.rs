fn main() {
    let b = 5;
//---START
    //As everything's an expression, we get some fun results
    //If statements:

    let a = if (b > 0) {
        "Positive"
    } else {
        "Negative"
    };

    //We also get pattern matching
    let c = match b {
        3 | 5 => "Lol",
        _ => "Not lol"
    };
//---END
}