fn main() {
    // integers
    let int: i32 = 31;      // 32 bit signed
    let uint: u32 = 31;     // 32 bit unsigned

    println!("int32: {int}");
    println!("uint32: {uint}");
    
    // float
    let float: f32 = 31.31; // 32 bit float
    println!("float: {float}");

    // boolean
    let boolean: bool = false;
    println!("boolean: {boolean}");

    // char
    let character: char = 'w';
    println!("character: {character}");

    // tuple
    let human: (u8, f32, f32) = (20, 75.3, 178.0); // age, weight, lenght
    println!("tuple: {:#?}", human);
    println!("age: {}, weight: {}, lenght: {}", human.0, human.1, human.2);

    // array
    let array: [i32; 5] = [31, 32, 33, 34, 35];
    println!("array = {:#?}", array);

    let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    
    println!("months = {:#?}", months);

    // string
    let string = "Hello Rust!";
    println!("string: {}", string);
}
