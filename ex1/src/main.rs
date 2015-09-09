use std::io;

fn main() {
	println!("CryptoPals Rust");
	println!("Enter Hex Number:");
	let mut hex_input = String::new();	
	io::stdin().read_line(&mut hex_input)
	.ok()
	.expect("IO Error Unable to read line");
	print!("Input was {}",hex_input);
	let bytes = hex_input.as_bytes();
	for b in bytes{
		println!("{:x}",b);
	}
	//for a in hex_input.chars(){
	//println!("{} char",a);
	//}
}
