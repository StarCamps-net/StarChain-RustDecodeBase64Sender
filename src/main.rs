extern crate base64;
extern crate reqwest;

use base64::encode;
use std::io::{self, Write};
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    // Prompt the user to enter a message
    print!("Enter a message to encode and send: ");
    io::stdout().flush().unwrap();

    // Read the input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    // Encode the input
    //let encoded = encode(input);
    //println!("Encoded: {}", encoded);

    // Connect to the receiver
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            // Send the encoded message
            stream.write_all(input.as_bytes()).expect("Failed to send message");
            println!("Message sent");
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}