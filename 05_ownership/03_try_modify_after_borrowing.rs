// error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
fn main() {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}