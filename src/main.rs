// Orginally Created By jeesusmies under the terms of GNU General Public License v3.0

use std::env;
use std::fs::{self, File};
use std::io::ErrorKind;
use rand::{self, Rng, rngs};

// generates a password using the rand crate
// chars, pass_len are unnecessary, if gen_weak_pass is true
// will probably make seperate functions in the future
fn gen_pass(chars: String, words: Vec<String>, pass_len: u8, mut rng: rngs::ThreadRng, gen_weak_pass: bool) -> String {
	let mut s = String::new();

	if gen_weak_pass { 
		let word = &words[rng.gen_range(0, words.len())];
		s.push_str(&word);
		for _i in 0..4 {
			let rnd_num = rng.gen_range(0, 9);
			let num_string = format!("{}", rnd_num);
			s.push_str(&num_string);
		}
	} else {
		// adds a random character to the `s` variable pass_len times
		for _i in 0..pass_len + 1 {
			let rnd_num = rng.gen_range(0, chars.len());
			s.push_str(&chars[rnd_num..rnd_num+1]);
		}
	}
	s
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut rng = rand::thread_rng();
	let chars = String::from("abcdefghijklmnopqrstuwxyzABCDEFGHIJKLMNOPQRSTUWXYZ1234567890!#%&/()=+?");
	// please tell me how to make this line better
	let words: Vec<String> = vec!["doggy".to_string(), "player".to_string(), "batman".to_string(), "catlover".to_string(), "doglover".to_string(), "cake".to_string(), "qwerty".to_string(), "active".to_string()];
	
	// arguments
	// maybe will add a config file to change default values easily?
	let mut service: String = String::from("undefined");
	let mut pass_len: u8 = 8;
	let mut file_name: String = String::from("passwords.txt"); 
	let mut gen_weak_pass: bool = false;
	
	// handles the command line arguments
	for (i, item) in args.iter().enumerate() {
		match item.as_str() {
			"-l" => pass_len = { let temp = &args[i+1];
								temp.parse().expect("went wrong..") },
			"-s" => service = { let temp = &args[i+1];
								temp.to_string() },
			"-o" => file_name = { let temp = &args[i+1];
								temp.to_string() },
			"-gwp" => gen_weak_pass = true,
			_ => ()
		}
	}

	// formats/joins (?) the password and service into a one single string
	let data = format!("Password: {}\nService: {}\n\n",gen_pass(chars, words, pass_len, rng, gen_weak_pass), service);
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
}
