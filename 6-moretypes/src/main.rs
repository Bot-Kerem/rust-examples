
type Name = String;
type Age = u8;

fn main()
{
    // casting
    let uint16 = 1023i16;
    let uint8 = uint16 as u8;

    println!("u16 to u8: {}", uint8); // output is 255
                                      // uint16 -> 000000001111111111
                                      // uint8  ->           11111111
    
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // vectors (dynamic lists)
    let mut vector = Vec::new();
    
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);

    println!("vector: {:?}", vector);

    // typedef
    let my_name: Name = String::from("Kerem");
    let my_age: Age = 23;

    println!("My name is {}. I'm {} years old.", my_name, my_age);
}
