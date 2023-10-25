use std::env;

mod bencoding;

use bencoding::bencode_decode;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("You didn't give me anything to decode.");
    } else {
        let (val, _) = bencode_decode(&args[1]);

        println!("Decoded value: {:?}", val);
    }
}
