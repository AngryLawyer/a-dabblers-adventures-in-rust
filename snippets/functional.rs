fn main() {
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

}
