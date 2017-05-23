extern crate rustc_serialize as serial;
use serial::base64::{self, ToBase64};
use serial::hex::FromHex;
fn main() {
println!("Exercise 1: Hex -> Base64"); 
let orig = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
let res = orig.from_hex().unwrap().to_base64(base64::STANDARD);
println!("Original String String {}",orig);
println!("Base64 Encoded String {}",res);
}
