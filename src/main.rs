use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::ErrorKind;
use rand::{self, Rng, rngs};

// generates a password using the rand crate
fn gen_pass(chars: String, pass_len: u8, mut rng: rngs::ThreadRng) -> String {
	let mut s = String::new();
	
	// adds a random character to the `s` variable pass_len times
	for _i in 0..pass_len + 1 {
		let rnd_num = rng.gen_range(0, chars.len());
		s.push_str(&chars[rnd_num..rnd_num+1])
	}

	s
}

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	
	let mut rng = rand::thread_rng();
	let chars = String::from("abcdefghijklmnopqrstuwxyzABCDEFGHIJKLMNOPQRSTUWXYZ1234567890!#%&/()=+?");

	let mut service: String = String::from("undefined");
	let mut pass_len: u8 = 8;
	let mut file_name: String = String::from("passwords.txt");
	
	// handles the command line arguments
	for (i, item) in args.iter().enumerate() {
		match item.as_str() {
			"-l" => pass_len = { let temp = &args[i+1];
								temp.parse().expect("went wrong..") },
			"-s" => service = { let temp = &args[i+1];
								temp.to_string() },
			"-o" => file_name = { let temp = &args[i+1];
								temp.to_string() },
			_ => ()
		}
	}
	
	// formats/joins (?) the password and service into a one single string
	let data = format!("Password: {}\nService: {}\n\n",gen_pass(chars, pass_len, rng), service);
	let f = File::open(file_name.as_str());
	
	// matches if `f` returns Err or Ok
	let mut f = match f {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create(file_name.as_str()) {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating file: {:?}", e)
			},
			other_error => panic!("Problem opening file: {:?}", other_error)
		}
	};

	// writes the data into the file
	fs::write({
		let temp = &file_name;
		temp.as_str()
		}, {
			let temp = format!("{}{}",{
				let temp = &file_name;
				fs::read_to_string(temp).expect("Unable to write file")
		}, data.as_str());
		temp
	});
	
	println!("Created a password and stored it in {}", file_name);
	Ok(())
}
