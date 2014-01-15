fn main() {
    let b = 5;
//---START:1
    let a = if (b > 0) {
        "Positive"
    } else {
        "Negative"
    };
//---END:1
//---START:2
    let c = match b {
        3 | 5 => "Lol",
        _ => "Not lol"
    };
//---END:2
}
