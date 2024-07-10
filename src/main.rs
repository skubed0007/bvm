pub mod bbmain;
//pub mod utils;

use bbmain::start;
use clearscreen::clear;
use colored::Colorize;
use std::{env::args, fs};

fn main() {
    let mut bytes = Vec::new();
    let f: Vec<String> = args().collect();

    if f.len() < 2 {
        eprintln!("{}", "Error: No file provided.".red());
        std::process::exit(1);
    }

    let bxe = &f[1];

    let bindat = match fs::read_to_string(bxe) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}{}", "Error reading file: ".red(), e.to_string().red());
            std::process::exit(1);
        }
    };

    for bin_str in bindat.split('`') {
        if bin_str.is_empty() {
            continue;
        }

        let byte = match u8::from_str_radix(bin_str, 2) {
            Ok(b) => b,
            Err(e) => {
                eprintln!(
                    "{}{}{}{}",
                    "Error parsing binary string '".red(),
                    bin_str.red(),
                    "': ".red(),
                    e.to_string().red()
                );
                std::process::exit(1);
            }
        };

        let original_byte = if byte == 0 {
            u8::MAX
        } else {
            byte.wrapping_sub(1)
        };

        bytes.push(original_byte);
    }

    let dat = match String::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "{}{}",
                "Error converting bytes to string: ".red(),
                e.to_string().red()
            );
            std::process::exit(1);
        }
    };

    if let Err(e) = clear() {
        eprintln!("{}{}", "Error clearing screen: ".red(), e.to_string().red());
    }

    //println!("data - {}", dat);
    let sepdat = dat.split("@");
    let mut i = 0;
    for dat in sepdat {
        if i == 1 {
            start(dat.to_string());
        }
        else{
            //TODO : print credits
            i += 1;
        }
    }
}
