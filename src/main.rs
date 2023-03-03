use std::io;
use clap::{Command, Arg, ArgAction};

fn main() -> io::Result<()> {
    let args = Command::new("txsplit")
                    .author("Anthony, ethanrcox@protonmail.com")
                    .version("1.0")
                    .about("Splits text on a delimiter")
                    .arg(
                        Arg::new("delimiter")
                    ).get_matches();

    for line in io::stdin().lines() {
        let line = line?;
    }

    Ok(())
}
