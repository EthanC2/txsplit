use std::io;
use clap::{Command, Arg, ArgAction};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let args = Command::new("txsplit")
                    .author("Anthony, ethanrcox@protonmail.com")
                    .version("1.0")
                    .about("Splits text on a delimiter")
                    .arg(
                        Arg::new("input-delimiter")
                            .long("input-delimiter")
                            .short('d')
                            .help("the delimiter(s) to split the input on")
                    )
                    .arg(
                        Arg::new("output-delimiter")
                        .long("output-delimiter")
                        .short('D')
                        .help("the delimiter to print after each split entry")
                    ).get_matches();

    let input_delimiters: HashSet<char> = HashSet::from_iter(args.get_one::<String>("input-delimiter").unwrap_or(&String::from(" \t\n\r")).chars());
    let output_delimiter = args.get_one::<String>("output-delimiter").unwrap_or(&String::from("\n"));

    for line in io::stdin().lines() {
        let line = line?;
        let mut lptr = 0;

        for (rptr, ch) in line.chars().enumerate() {
            lptr = rptr;            
        }
    }

    Ok(())
}
