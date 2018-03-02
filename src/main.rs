extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::env;
use std::io::{self, BufRead};
use std::u8;
use std::usize;

fn hexify(s : &str) -> String {
    let mut out = String::new();
    let bytes = s.as_bytes();

    for &b in bytes {
        out.push_str(&format!("{:02x}", &b));
    }

    return out;
}

fn dehexify(s : &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len() / 2);

    for i in 0..(s.len() / 2) {
        let hex = u8::from_str_radix(&s[i*2..i*2+2], 16)
                    .expect("Hex parse failure");
        out.push(hex);
    }

    return out;
}

fn mangle(name : &str) -> String {
    let mut offset = 0;
    let mut out = String::with_capacity(2 * name.len()); // heuristic
    lazy_static! {
        static ref RE_TOKEN    : Regex = Regex::new(r"^(?i)[a-z_]\w*").unwrap();
        static ref RE_NONTOKEN : Regex = Regex::new(r"^[0-9]*\W*").unwrap();
    }

    out.push('_');

    while offset < name.len() {
        match RE_TOKEN.find(&name[offset..]) {
            Some(m) => {
                let s = m.as_str();
                let len = s.len();
                offset += len;
                out.push_str(&format!("{}{}", len, s));
            },
            None => {},
        };
        match RE_NONTOKEN.find(&name[offset..]) {
            Some(m) if m.as_str().len() > 0 => {
                let s = m.as_str();
                let len = s.len();
                offset += len;
                out.push_str(&format!("0{}_{}", len, hexify(&s)));
            },
            _ => {},
        };
    }

    out.shrink_to_fit();
    return out;
}

fn demangle(name : &str) -> String { // TODO Option<String>
    let mut offset = 0;
    let mut out = String::with_capacity(name.len());
    lazy_static! { static ref NUM : Regex = Regex::new(r"^\d+").unwrap(); }

    if &name[0..1] != "_" {
        panic!("Bad identifier (expected `_`)");
    } else {
        offset += 1;
    }

    let mut is_hex = false;
    while offset < name.len() {
        match &name[offset..offset+1] {
            "0" => {
                offset += 1;
                is_hex = true;
            },
            _ => {
                match NUM.find(&name[offset..]) {
                    Some(m) => {
                        let len = usize::from_str_radix(m.as_str(), 10)
                                    .expect("Hex parse failure");
                        offset += m.as_str().len();
                        if is_hex {
                            if &name[offset..offset+1] != "_" {
                                panic!("Bad identifier (expected `_`)");
                            }
                            offset += 1;
                            let nybbles = 2 * len;
                            let vec = dehexify(&name[offset..offset+nybbles]);
                            let utf8 = String::from_utf8(vec)
                                        .expect("Expected UTF-8 string");
                            out.push_str(&utf8);
                            offset += nybbles;
                        } else {
                            out.push_str(&name[offset..offset+len]);
                            offset += len;
                        }
                        is_hex = false;
                    },
                    None => panic!("Bad identifier (expected number)"),
                };
            },
        }
    }

    out.shrink_to_fit();
    return out;
}

fn main() {
    let stdin = io::stdin();
    let args : Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Need `--mangle` or `--demangle` option");
    }
    let func = match &args[1][..] {
        "--mangle" => mangle,
        "--demangle" => demangle,
        _ => panic!("Invalid option `{}`", &args[1]),
    };
    for line in stdin.lock().lines() {
        println!("{}", func(&line.expect("Line read failure")));
    }
}

