use std::process::Command;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


fn main() {
    // Why doesn't this work?
    let bob = Person{ name:"Robert", age:42 };

    // Print text to the console
    let x = 5 + 2;
    println!("Hello world. 100? x = {} and Bob is:\n{:#?}", x, bob);

    match x {
        1 => println!("Hello!"),
        2 => println!("Hello 2!"),
        3 => println!("Hello3 !"),
        3...7 => println!("Between 3 and 7!"),
        _ => println!("Fizzbuzz"),
    }
    let y = 1..10;
    println!("The numbers between 1 and 10 are {:?}", y);

    let cmd_output = Command::new("head")
        .arg("/proc/cpuinfo")
        .output()
        .expect("failed to execute process")
        .stdout;

    let date_str = String::from_utf8(cmd_output).expect("Not valid UTF");
    println!("The CPU info is {}", date_str);
}

