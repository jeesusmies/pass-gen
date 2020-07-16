# pass-gen
Password generator I made in python rewritten in rust, with some added features.

# How to use
1. `cargo new pass-gen`
2. Move the `src/main.rs` and `Cargo.toml` into your `pass-gen` directory
3. `cargo build`
4. Go into `pass-gen/target/debug` and you can see the executable as `pass-gen` or `pass-gen.exe`.
5. Run it by executing the file in your terminal

# Arguments
`-o`: output file (default: passwords.txt)  
`-l`: password length (default: 8)  
`-s`: service used (default: undefined)  
&nbsp;  &nbsp;  &nbsp;  &nbsp;  ^-- (e.g youtube)
