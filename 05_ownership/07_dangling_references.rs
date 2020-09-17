fn main() {
    let reference_to_nothing = dangle();    // error!
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}   // s is dropped, so it's memory goes away.