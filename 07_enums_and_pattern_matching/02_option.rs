// Option : for null case handling
// https://doc.rust-lang.org/std/option/enum.Option.html
fn main(){
    enum Option<T> {    // <T> syntax : generic type parameter
        Some(T),
        None,           // for null
    }
}