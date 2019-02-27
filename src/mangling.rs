extern crate rand;

#[cfg(test)]
use rand::distributions::{Distribution, Normal, Standard};
#[cfg(test)]
use rand::{thread_rng, Rng};

use std::str::FromStr;

#[cfg(test)]
const MANGLE_LIST : &[(&str, &str)] = &[
    ( "()V"                        , "_02_28291V"                                            ),
    ( "(II)I"                      , "_01_282II01_291I"                                      ),
    ( "<init>"                     , "_01_3c4init01_3e"                                      ),
    ( "<init>:()V"                 , "_01_3c4init04_3e3a28291V"                              ),
    ( "Code"                       , "_4Code"                                                ),
    ( "GCD"                        , "_3GCD"                                                 ),
    ( "StackMapTable"              , "_13StackMapTable"                                      ),
    ( "gcd"                        , "_3gcd"                                                 ),
    ( "java/lang/Object"           , "_4java01_2f4lang01_2f6Object"                          ),
    ( "java/lang/Object.<init>:()V", "_4java01_2f4lang01_2f6Object02_2e3c4init04_3e3a28291V" ),
];

#[test]
fn test_mangle() {
    for (unmangled, mangled) in MANGLE_LIST {
        assert_eq!(&mangle(unmangled.as_ref()), mangled);
    }
}

pub fn mangle(name : &[u8]) -> String {
    let mut out = String::with_capacity(2 * name.len()); // heuristic

    out.push('_');

    let begin_ok = |x : char| x.is_ascii_alphabetic() || x == '_';
    let within_ok = |x : char| begin_ok(x) || x.is_ascii_digit();

    let mut remain = name.iter().peekable();

    loop {
        let mut v = Vec::new();
        match remain.peek() {
            Some(&&r) if begin_ok(char::from(r)) => {
                loop {
                    match remain.peek() {
                        Some(&&r) if within_ok(char::from(r)) => {
                            v.push(r);
                            remain.next();
                        },
                        _ => break,
                    }
                }
                out.push_str(&format!("{}{}", v.len(), String::from_utf8(v).unwrap()));
            },
            Some(_) => {
                loop {
                    match remain.peek() {
                        Some(&&r) if !begin_ok(char::from(r)) => {
                            v.push(r);
                            remain.next();
                        },
                        _ => break,
                    }
                }
                out.push_str(&format!("0{}_{}", v.len(), hexify(v.as_ref())));
            },
            _ => {
                out.shrink_to_fit();
                return out;
            }
        }
    }
}

#[test]
fn test_demangle() {
    for (unmangled, mangled) in MANGLE_LIST {
        let got : Vec<u8> = demangle(mangled);
        let want : Vec<u8> = unmangled.to_owned().to_string().into();
        assert_eq!(want, got);
    }
}

pub fn demangle(name : &str) -> Vec<u8> { // TODO Option<Vec<u8>>
    let mut out = Vec::with_capacity(name.len());

    let mut name = &name[..];
    if &name[0..1] != "_" {
        panic!("Bad identifier (expected `_`)");
    } else {
        name = &name[1..];
    }

    let mut is_hex = false;
    while !name.is_empty() {
        if &name[..1] == "0" {
            name = &name[1..];
            is_hex = true;
        }
        let mut not_num = 0;
        while char::from(*name.as_bytes()[not_num..].first().unwrap()).is_ascii_digit() {
            not_num += 1;
        }
        let len = usize::from_str(&name[..not_num]).unwrap();
        name = &name[not_num..];
        if is_hex {
            if &name[..1] != "_" {
                panic!("Bad identifier (expected `_`)");
            }
            name = &name[1..];
            let nybbles = 2 * len;
            out.append(&mut dehexify(&name[..nybbles]));
            name = &name[nybbles..];
        } else {
            out.append(&mut Vec::from(&name[..len]));
            name = &name[len..];
        }
        is_hex = false;
    }

    out.shrink_to_fit();
    return out;
}

#[test]
fn test_round_trip() {
    let mut rng = thread_rng();
    let norm = Normal::new(20.0, 5.0);
    for i in 1..10 {
        let len = norm.sample(&mut rng) as usize;
        let rs : Vec<u8> = rng.sample_iter(&Standard).take(len).collect();

        assert_eq!(rs, demangle(&mangle(&rs.as_ref())));
    }
}

fn hexify(bytes : &[u8]) -> String {
    let mut out = String::new();

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
