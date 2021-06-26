use clap::{App, Arg};

fn main() {
    let matches = App::new("passphrase")
        .arg(
            Arg::with_name("wordlist")
                .long("wordlist")
                .takes_value(true)
                .default_value("assets/eff_large_wordlist.txt"),
        )
        .arg(
            Arg::with_name("count")
                .long("count")
                .takes_value(true)
                .default_value("6"),
        )
        .arg(
            Arg::with_name("seperator")
                .long("seperator")
                .takes_value(true)
                .default_value(" "),
        )
        .arg(Arg::with_name("capital").long("capital"))
        .get_matches();
    let wordlist = matches.value_of("wordlist").unwrap();
    let wordcount: u32 = matches.value_of("count").unwrap().parse().unwrap_or(6_u32);
    let seperator: &str = matches.value_of("seperator").unwrap();
    let want_camel = matches.is_present("capital");
    match passphrase::run(wordlist, wordcount, seperator, want_camel) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
