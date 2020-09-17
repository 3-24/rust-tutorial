fn main() {
    let some_u8_value = Some(0u8);
    
    // Handling only Some case with placeholder
    /*
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("This is not three")
    }*/

    // Using if let
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("This is not three");
    }
}