fn main() {
    let s0 = String::from("hello");
    takes_ownership(s0);
    // println!("{}",s0); // error[E0382]: borrow of moved value: `s0`

    let s1 = gives_ownership();
    println!("{}", s1);
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}",s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_ownership(a_string: String) {}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
