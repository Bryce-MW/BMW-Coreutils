use clap::{Arg, App};

fn main() {
    let matches = App::new("echo")
        .version("0.1.0")
        .author("Bryce Wilson <bmw_coreutils@brycemw.ca>")
        .about("Writes arguments to standard out")
        .arg(Arg::with_name("no-newline")
            .short("n")
            .help("Do not print a newline character"))
        .arg(Arg::with_name("strings")
            .value_name("STRING")
            .multiple(true)
            .help("The strings to print"))
        .get_matches();

    if let Some(strings) = matches.values_of("strings") {
        let mut last = false;
        for string in strings {
            if last {
                print!(" ");
            } else {
                last = true;
            }
            print!("{}", string);
        }
    }
    if !matches.is_present("no-newline") {
        println!();
    };
}
