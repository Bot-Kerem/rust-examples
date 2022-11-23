
#![allow(dead_code)]

enum Vec3
{
    RGB(i32, i32, i32),
    XYZ(i32, i32, i32),
}

enum Temp
{
        Celsius(i32),
        Fahrenheit(i32),
}

enum Pet
{
    Dog,
    Cat,
    Fish
}

fn get_age() -> u32
{
    7
}

fn main() {
    // if else
    let i = 31;
    if i == 31
    {
        println!("It's magic number");
    }
    else if i == 69
    {
        println!("It's magic number");
    }
    else
    {
        println!("Isn't magic number");
    }

    println!("~~~~~~~~~~~~~~~~");
    let mut count = 0;
    // loops
    loop // infinite loop
    {
        count += 1;
        if count == 16 { break; }
        
        if (count % 15) == 0
        {
            println!("FizzBuzz");
        }
        else if (count % 3) == 0
        {
            println!("Fizz");
        }
        else if (count % 5) == 0
        {
            println!("Buzz");
        }
        else
        {
            println!("{count}");
        }
    }

    // interesting shit
    'outer: loop
    {
        'inner: loop
        {
            if 1 == 2
            {
                break 'inner;
            }
            break 'outer; // you can use break for different scopes
        }
    }
    
    println!("~~~~~~~~~~~~~~~~");
    // returning from loops

    let mut counter = 0i32;
    let mut val = 1i32;
    let factorial = loop // 5!
    {
        counter += 1;
        if counter == 6
        {
            break val;
        }
        val *= counter;
    };
    println!("factorial: {factorial}");

    println!("~~~~~~~~~~~~~~~~");
    // while loop
    let mut n = 0;
    while n < 5
    {
        n += 1;
        println!("n: {n}");
    }

    println!("~~~~~~~~~~~~~~~~");
    // for loops
    for n in 1..5
    {
        println!("n: {n}");
    }
    println!("~~~~~~~~~~~~~~~~");
    // 5 inclusive
    for n in 1..=5
    {
        println!("n: {n}");
    }
    println!("~~~~~~~~~~~~~~~~");
    let names = vec!["Rust", "C++", "Python"];

    for name in names.iter() {
        match name {
            &"Rust" => println!("Rust sucks!"),
            _ => println!("{} is gut", name),
        }
    }
    
    println!("names: {:?}", names);
    println!("~~~~~~~~~~~~~~~~");
   
    // match usage in different types
    // for tuples
    let quadric = (0, 1, 2, 3); // change elements
    match quadric {
        (0, x, y ,z) => println!("x: {x}, y: {y}, z: {z}"),
        (2, ..) => println!("The first element is 2!"),
        (.., 5) => println!("The last element is 5!"),
        (8, .., 7) => println!("The first element is 8 and the last is 7!"),
        _   => println!("Default")
    }

    // for arrays
    let arr = [4, 2 ,82 ,4 ,8];
    match arr {
        [1, a, b, ..] => println!("arr[1]: {a}, arr[2]: {b}"),
        [2, .., a, _] => println!("a: {a}"),
        [_, middle @ .., _] => println!("middle : {:?}", middle)
    }
    // for enums
    println!("~~~~~~~~~~~~~~~~");
    let point = Vec3::XYZ(5, 2, 4);
    match point
    {
        Vec3::XYZ(x, y, z) => println!("x; {x}, y: {y}, z: {z}"),
        Vec3::RGB(r, g, b) => println!("r; {r}, g: {g}, b: {b}"),
    }

    // for reference and pointer
    println!("~~~~~~~~~~~~~~~~");

    let value = &8;
    match value
    {
        &val => println!("val: {}", val)
    }
    match *value
    {
        val => println!("Val: {:?}", val),
    }

    let mut reference = 2;
    match reference
    {
        ref mut val =>{ *val+=2;println!("Val: {:?}", val); },
    }
    println!("Ref: {:?}", reference);
    println!("~~~~~~~~~~~~~~~~");
    // guards
    let temp = Temp::Celsius(38);
    match temp
    {
        Temp::Celsius(t) if t > 37 => println!("You are sick"),
        Temp::Celsius(_t) => println!("You are healty"), // I hope
        Temp::Fahrenheit(t) if t > 100 => println!("You are sick"),
        Temp::Fahrenheit(_t) => println!("You are healty"),
    }
    println!("~~~~~~~~~~~~~~~~");

    // binding
    match get_age() {
        n @ 0 ..= 12 => println!("I'm child of age: {n}"),
        n @ 13 ..= 19 => println!("I'm teen of age: {n}"),
        n @ 20.. => println!("I'm booomer of age: {n}"),
    }
    let pet = Pet::Dog;

    if let Pet::Dog = pet {
        println!("Your pet is a dog");
    }

    // while let
    let mut timer = Some(5);
    while let Some(n) = timer
    {
        if timer == Some(0) {
            println!("Time's up!");
            timer = None;
        }
        else
        {
            println!("{:?} loops left...", timer.unwrap());
            timer = Some(n - 1);
        }
        
    }
}

