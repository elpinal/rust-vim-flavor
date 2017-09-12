extern crate vim_flavor;

fn main() {
    let r = vim_flavor::install("flavor 'vspec'");
    if let Some(e) = r.err() {
        println!("{}", e);
    }
}
