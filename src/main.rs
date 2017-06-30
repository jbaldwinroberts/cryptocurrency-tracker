use std::collections::HashMap;
use std::io::Read;

extern crate strfmt;
use strfmt::strfmt;

extern crate hyper;
use hyper::Client;

extern crate clap;
use clap::{App, Arg};

extern crate serde;
extern crate serde_json;

fn main() {
    let matches = App::new("rust-crypto-tracker")
        .version("0.1.0")
        .author("Joe Roberts <joe@resin.io>")
        .about("Get crypto stock information from coinmarketcap.com")
        .arg(
            Arg::with_name("crypto")
                .short("c")
                .long("crypto")
                .help("Enter the crypto(s)")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .help("Enter the format")
                .takes_value(true)
                .default_value(
                    "{name}, {symbol}, {rank}, {price_usd}, {price_btc}, \
                     {24h_volume_usd}, {market_cap_usd}, {available_supply}, \
                     {total_supply}, {percent_change_1h}, {percent_change_24h}, \
                     {percent_change_7d}, {last_updated}",
                ),
        )
        .arg(
            Arg::with_name("separator")
                .short("s")
                .long("separator")
                .help("Enter the separator")
                .takes_value(true)
                .default_value(" | "),
        )
        .get_matches();

    let cryptos: Vec<_> = matches.values_of("crypto").unwrap().collect();
    let format: &str = matches.value_of("format").unwrap();
    let separator: &str = matches.value_of("separator").unwrap();

    let client = Client::new();
    let mut response = client
        .get("http://api.coinmarketcap.com/v1/ticker/?limit=10")
        .send()
        .unwrap();
    let mut contents = String::new();
    response.read_to_string(&mut contents).unwrap();

    let stocks: Vec<HashMap<String, String>> = serde_json::from_str(&contents).unwrap();
    let stocks: HashMap<String, HashMap<String, String>> = stocks
        .iter()
        .map(|s| (s["symbol"].clone(), s.clone()))
        .collect();

    let mut iter = cryptos.iter().peekable();
    while let Some(current) = iter.next() {
        let stock = stocks.get::<str>(current).unwrap();
        print!("{}", strfmt(format, stock).unwrap());
        match iter.peek() {
            Some(_) => print!("{}", separator),
            None => println!(""),
        }
    }
}
