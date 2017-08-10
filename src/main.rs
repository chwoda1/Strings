// Author: Christopher Wood
// Date: 8/9/2017
//
// Rust implementation of MyStrings program
// from my CS449 operating systems class.

use std::env;
use std::path::Path;
use std::vec::Vec;
use std::fs::File;
use std::io::Read;

fn main(){

    let mut counter = 0;

    let arguments: Vec<String> = env::args().collect();

    if arguments.len() == 1{
        println!("Please Enter A File to Parse");
        return;
    }

    let path = Path::new(&arguments[1]);

    let file = match File::open(path){

        Ok(file) => file,
        Err(file) => panic!("Please Input a Valid File"),

    };

    //let mut buffer = Vec::<u8>::new();
    //
    // ERRORS: Change ASCII Table around

    let mut vector_array: Vec<u8> = Vec::new();

    for bytes in file.bytes(){
        let value = bytes.unwrap();

        if value < 0x7d && value >= 0x20{
            &vector_array.push(value);
            counter += 1;
        }
        else if (value > 0x7d || value <= 0x20 ) && counter < 4{
            &vector_array.clear();
            counter = 0;
        }
        else if (value > 0x7d || value <= 0x20) && counter >= 4{
            let array = vector_array.clone();
            let values = String::from_utf8(array).unwrap();
            println!("{}" , values);
            &vector_array.clear();
            counter = 0;
        }
    }
}
