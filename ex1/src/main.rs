use std::io;
use std::io::prelude::*;
fn main() {
    println!("CryptoPals Rust");
    println!("Enter Hex Number:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            //   println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }

    for a in input.chars(){
        println!("{} char",a);
    }
}
