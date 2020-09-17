fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,   // MUST cover this case. Otherwise the compiler warns it.
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
