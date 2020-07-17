# pass-gen
Simple Password generator written in rust, with some additional features.


## How to use
```
git clone https://github.com/jeesusmies/pass-gen ; cd pass-gen
cargo build --release
cd target/release
./pass-gen <arguments> (Unix)
pass-gen.exe <arguments> (Windows)
```

## Arguments
`-gwp`: generate weak password (default: false)  
`-o`: output file (default: passwords.txt)  
`-l`: password length (default: 8)  
`-s`: service used (default: undefined)  
&nbsp;  &nbsp;  &nbsp;  &nbsp;  &nbsp;  ^-- e.g youtube  
  
**If generating a weak password, `-l` is unnecessary.**


## License :scroll:
This Project is licensed under the GNU General Public License v3.0. Check `LICENSE` file for more info.
