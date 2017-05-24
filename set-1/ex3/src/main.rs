extern crate rustc_serialize as serial;
use serial::hex::FromHex;
extern crate ascii;
use ascii::AsciiChar;
use std::process;
use ascii::ToAsciiChar;
use std::str;
fn main() {
    println!("Single Byte XOR Cipher");
    let ctx = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        .from_hex()
        .unwrap();
    //println!("{:?}",ctx);
		#[derive(Debug, Copy, Clone)]
    let mut tmp_vec: Vec<char> = Vec::new();
		#[derive(Debug, Copy, Clone)]
    let res = "";
    for y in 0..128 {
		println!("y is: {}",y);
		let len = ctx.len()-1;
        for x in 0..len {
						println!("x is: {}",x);
            let tmp = y as u8 ^ ctx[x];
            //println!("{:b}",tmp);
            let tmp_char = tmp as char;
            /*            println!("{:?}", tmp_char.to_ascii_char().unwrap().is_graph());
            println!("{:?}",
                     tmp_char.to_ascii_char().unwrap().as_printable_char());*/
            if tmp_char.to_ascii_char().unwrap().is_print() {
                tmp_vec.push(tmp_char.to_ascii_char().unwrap().as_printable_char());
                let res: String = tmp_vec.clone().into_iter().collect();
                println!("{}", res);
            } else {
                println!("Invalid UTF");
            }
        }
								tmp_vec = Vec::new()
    }
    //println!("{}", tmp_vec.len());
    println!("--------------FIN------------------------");
}
