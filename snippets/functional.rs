fn main() {
//---START
    //We can get algerbraic data types!
    enum MaybeInt {
        SomeInt(int),
        NoneInt
    };
    //As the language doesn't have nulls, this is how we deal with nulls
    //Although, there's a generic Option type for this, which takes any type
    enum Option<T> {
        Some(T),
        None
    };

    //We can then unpack them with pattern matching
    let lol: Option<int> = Some(5);

    match lol {
        Some(x) => x,
        None => 0
    };

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
//---END
}
