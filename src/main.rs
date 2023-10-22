mod bencoding;

use bencoding::{bencode_decode, BencodingValue};

fn main() {
    println!("Test input 0:");

    let val = bencode_decode("0:");

    if let BencodingValue::String(s) = val {
        println!("Value is string: {s}");
    } else if let BencodingValue::Integer(i) = val {
        println!("Value is integer: {i}");
    }
}
