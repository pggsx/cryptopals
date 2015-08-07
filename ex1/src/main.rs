use std::io;

fn main() {
	println!("CryptoPals Rust");
	println!("Enter Hex Number:");
	let mut hex_input = String::new();
	
	io::stdin().read_line(&mut hex_input)
	.ok()
	.expect("IO Error Unable to read line");
	print!("Hex Input was : {}",hex_input);
	let mut hex_array[] = hex_input.as_bytes();
	
	for x in 0..array.len(){
		println!("{}",array[x]);
	}

}
