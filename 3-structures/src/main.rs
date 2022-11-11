#[derive(Debug)]
struct Human {
    name: String,
    age: u8,
}

fn main() {
    let human = Human { name: String::from("Kerem"), age:23 };
    println!("{:?}", human);
    println!("Name: {}, Age: {}", human.name, human.age);

}
