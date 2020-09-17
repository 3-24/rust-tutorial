fn main() {
    let s0 = String::from("hello");
    let (len, s0) = calculate_length_without_pointer(s0);
    println!("The length of '{}' is {}.", s0, len);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);    // pass variable by a refence to s1!
    println!("The length of '{}' is {}.", s1, len);
}

// Returing input via tuple
fn calculate_length_without_pointer(s: String) -> (usize, String) {
    let length = s.len();
    (length, s)
}

// Using reference - 'borrowing'
// It is not takeing the ownership of s, hence we don't drop s.
fn calculate_length(s: &String) -> usize {
    s.len()
}