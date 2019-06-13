use std::process::Command;
use working_ffmpeg::foo;
use std::io::{self, BufRead};

use rand::Rng;


#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


fn main() {
    //let mut rng = rand::thread_rng();
    //
    println!("Enter a filename:");

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let input_filename = iterator.next().unwrap().unwrap();

    let mut output_filename = input_filename.clone();
    output_filename.push_str(".mp4");

    foo(&input_filename, &output_filename);

    let x = 2;
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

