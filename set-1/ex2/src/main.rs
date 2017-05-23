extern crate rustc_serialize as serial;
use serial::hex::{FromHex,ToHex};
fn main(){
println!("Exercise 2: Fixed XOR");
let s = "1c0111001f010100061a024b53535009181c";
let xr = "686974207468652062756c6c277320657965";
let s_vec = s.from_hex().unwrap();
let xr_vec = xr.from_hex().unwrap();
let mut xor_fin:Vec<u8> = Vec::new();
if s_vec.len() % 2 == 0 && xr_vec.len() % 2 == 0 && xr_vec.len() == s_vec.len(){
for i in 0..s_vec.len() {
let xr_tmp = s_vec[i] ^ xr_vec[i];
xor_fin.push(xr_tmp);
}

}
//let fin =  String::from_utf8_lossy(xor_fin).to_string().to_hex();
let fin = xor_fin.to_hex();
println!("Original {}",s);
println!("Post XOR {}",fin);
}
