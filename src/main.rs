use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

type TerminatingResult = std::result::Result<(), Box<std::error::Error>>;

fn translate_file(input_filename : PathBuf, output_filename : PathBuf) -> TerminatingResult {
    use std::io::Write;

    let stem = Path::new(&input_filename).with_extension("");
    let stem = stem.to_str().ok_or("expected Unicode filename")?;
    let class = classfile_parser::parse_class(&stem)?;

    let mut outfile = File::create(output_filename)?;
    for method in &class.methods {
        let mm = tyrga::translate_method(&class, method)?;
        writeln!(outfile, "{}", mm)?;
    }

    Ok(())
}

fn main() -> TerminatingResult {
    use clap::*;

    let m =
        app_from_crate!()
            .subcommand(
                SubCommand::with_name("translate")
                    .about("Translates JVM .class files into tenyr .tas assembly files")
                    .arg(Arg::with_name("classes")
                            .help("Names .class files as input")
                            .multiple(true)
                            .required(true)
                        )
                )
            .subcommand(
                SubCommand::with_name("mangle")
                    .about("Mangles strings of bytes into valid tenyr symbols")
                    .arg(Arg::with_name("strings")
                            .help("Provides string inputs for mangling")
                            .multiple(true)
                            .required(true)
                        )
                )
            .subcommand(
                SubCommand::with_name("demangle")
                    .about("Decodes mangled tenyr symbols into strings")
                    .arg(Arg::with_name("strings")
                            .help("Provides string inputs for demangling")
                            .multiple(true)
                            .required(true)
                        )
                )
            .get_matches();

    if let Some(m) = m.subcommand_matches("translate") {
        for file in m.values_of("classes").ok_or("expected at least one input file")? {
            let file = Path::new(&file);
            let out : PathBuf = file.with_extension("tas").file_name().ok_or("expected path to have a filename")?.into();
            println!("Creating {} from {} ...", out.display(), file.display());
            translate_file(file.to_owned(), out)?;
        }
    } else if let Some(m) = m.subcommand_matches("mangle") {
        for string in m.values_of("strings").ok_or("expected at least one string to mangle")? {
            println!("{}", tyrga::mangling::mangle(string.bytes())?);
        }
    } else if let Some(m) = m.subcommand_matches("demangle") {
        for string in m.values_of("strings").ok_or("expected at least one string to mangle")? {
            let de = tyrga::mangling::demangle(&string)?;
            let st = String::from_utf8(de)?;
            println!("{}", st);
        }
    }

    Ok(())
}

