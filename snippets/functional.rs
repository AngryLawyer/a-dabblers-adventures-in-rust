fn main() {
//---START:1
    //We can get algerbraic data types!
    enum MaybeInt {
        SomeInt(int),
        NoneInt
    };
    //As the language doesn't have nulls,
    //We need a way to represent an empty response.
    
    //Something more generic:
    enum Option<T> {
        Some(T),
        None
    };
//---END:1
//---START:2
    //Now, when we define a function,
    //We can mark that it might not give us anything back
    
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
//---START:3
    //So, let's do the classic of Functional Programming. Define a list type.
    enum List<T> {
        Node(T,~List<T>),
        Terminal
    };

    //So, now we can define a list!
    let list = ~Node(1, ~Node(2, ~Node(3, ~Terminal)));

    //And now we can write functions to fiddle with it
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

    //But wait, doubling a list seems a little silly when we could write a Map function!
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

    //Of course, in real Rust code, you'd use the built in types rather than redefine these basics
//---END:3
}
