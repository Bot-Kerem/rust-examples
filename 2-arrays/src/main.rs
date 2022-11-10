fn main() {
    let arr: [i32; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    println!("first 3 elements: {:#?}", &arr[0 .. 3]); // borrow as a slice
    println!("size of array: {}", arr.len());

    for n in arr
    {
        if n % 15 == 0
        {
            println!("{}", "FizzBuzz");
        }
        else if n % 3 == 0
        {
            println!("{}", "Fizz");
        }
        else if n % 5 == 0
        {
            println!("{}", "Buzz");
        }
        else
        {
            println!("{}", n);
        }
    }
}
