extern crate vim_flavor;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let name = env::args().nth(1).expect("1 argument needed");
    let mut f = File::open(name).expect("cannot open Flavor file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect(
        "failed to read Flavor file",
    );
    let r = vim_flavor::install(&buffer);
    if let Some(e) = r.err() {
        println!("{}", e);
    }
}
