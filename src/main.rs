use clap::{App, Arg, ArgMatches};
use passphrase::Runtime;

fn make_runtime<'a>(matches: &'a ArgMatches) -> Runtime<'a> {
    Runtime {
        wordlist: matches.value_of("wordlist").unwrap(),
        wordcount: matches.value_of("count").unwrap().parse().unwrap_or(6_u32),
        seperator: matches.value_of("seperator").unwrap(),
        want_capital: matches.is_present("capital"),
    }
}

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
    match passphrase::run(&make_runtime(&matches)) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
