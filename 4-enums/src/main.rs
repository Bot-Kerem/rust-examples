#![allow(non_snake_case)]


enum Event
{
    Run(i32),
    Sleep,
    Eat(String),
    Read(String)
}

fn Do(event: Event)
{
    match event
    {
        Event::Run(c) => println!("{} meters run", c),
        Event::Sleep => println!("Slept"),
        Event::Eat(c) => println!("{} eaten", c),
        Event::Read(c) => println!("{} read", c),
    }
}

fn main()
{
    let run = Event::Run(1500); 
    let sleep = Event::Sleep; 
    let eat = Event::Eat("Doner".to_owned()); 
    let read = Event::Read("Listen Little Man".to_owned());

    Do(run);
    Do(sleep);
    Do(eat);
    Do(read);
}
