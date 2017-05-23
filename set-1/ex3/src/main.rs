extern crate rustc_serialize as serial;
use serial::hex::{FromHex,ToHex};
extern crate ascii;
//use ascii::ascii_string::{IntoAsciiString};
use ascii::IntoAsciiString;
fn main()
{
println!("Single Byte XOR Cipher");
let ctx = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap();
println!("{:?}",ctx);
let mut tmp_vec:Vec<u8> = Vec::new();
for x in 0..ctx.len() {
for y in 0..128{
let tmp = y ^ ctx[x];
if tmp >=0 && tmp <= 128{
tmp_vec.push(tmp);
let mut vec_str = tmp_vec.into_ascii_string();
println!("{:?}",vec_str);
}
//println!("{:b}",ctx[x]);
let test = format!("{:b}",ctx[x]).to_string();
//println!("{}",test);
}
}
}

