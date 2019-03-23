use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Lets simply open the Cargo.toml file
    let path = Path::new("Cargo.toml");
    let mut file = File::open(&path).expect("Failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect(
        "Failed to read from file",
    );
    print!("{}", s);

    let mut ofile = File::create(Path::new("f.txt")).expect("Could not create File");

    ofile.write_all(s.as_bytes()).expect("Failed to read");
}
