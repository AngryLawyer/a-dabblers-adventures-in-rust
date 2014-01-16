fn main() {
//---START:1-1
    enum MaybeInt {
        SomeInt(int),
        NoneInt
    };

    let maybe_number = SomeInt(5);
    let no_number = NoneInt;
//---END:1-1
//---START:1-2
    enum Option<T> {
        Some(T),
        None
    };
//---END:1-2
//---START:2
    fn safe_divide(a: int, b: int) -> Option<f64> {
        if b == 0 {
            None
        } else {
            Some(a as f64 / b as f64)
        }
    }

    //We can then unpack them with pattern matching
    match safe_divide(5, 0) {
        Some(x) => "Successfully divided",
        None => "Divide by zero"
    };
//---END:2
//---START:tut1
    enum List<T> {
        Node(T,~List<T>),
        Terminal
    };
//---END:tut1
//---START:tut2
    let list = ~Node(1, ~Node(2, ~Node(3, ~Terminal)));

    fn double_list(item: &List<uint>) -> ~List<uint> {
        match item {
            &Node(ref value, ref next) => {
                ~Node(value*2, double_list(*next))
            },
            &Terminal => {
                ~Terminal
            }
        }
    }
    let squared = double_list(list);
//---END:tut2
//---START:tut3
    fn map<T, U>(item: &List<T>, action: |&T| -> U) -> ~List<U> {
        match item {
            &Node(ref value, ref next) => {
                ~Node(action(value), map(*next, action))
            },
            &Terminal => {
                ~Terminal
            }
        }
    }

    //And now we can use it as normal
    let squared = map(list, |value: &uint| { *value * *value });
//---END:tut3
}
