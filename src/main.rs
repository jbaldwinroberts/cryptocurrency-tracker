use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;

extern crate strfmt;
use strfmt::strfmt;

extern crate hyper;
use hyper::Client;

extern crate clap;
use clap::{App, Arg};

extern crate serde;
extern crate serde_json;

mod tokenizer;

use tokenizer::{Token, TokenType, tokenise_format_str};

fn main() {
    let matches = App::new("rust-crypto-tracker")
        .version("0.1.0")
        .author("Joe Roberts <joe@resin.io>")
        .about("Get crypto stock information from coinmarketcap.com")
        .arg(
            Arg::with_name("crypto")
                .short("c")
                .long("crypto")
                .help("Enter the crypto")
                .takes_value(true)
                .default_value("ethereum"),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .help("Enter the format")
                .long_help("longer")
                .takes_value(true)
                .default_value(
                    "{name}, {symbol}, {rank}, {price_usd}, {price_btc}, \
                     {24h_volume_usd}, {market_cap_usd}, {available_supply}, \
                     {total_supply}, {percent_change_1h}, {percent_change_24h}, \
                     {percent_change_7d}, {last_updated}",
                ),
        )
        .get_matches();

    tokenise_format_str(matches.value_of("format").unwrap());

    let mut url = PathBuf::from("http://api.coinmarketcap.com/v1/ticker/");
    url.push(matches.value_of("crypto").unwrap());

    let client = Client::new();
    let mut response = client.get(url.to_str().unwrap()).send().unwrap();
    let mut contents = String::new();
    response.read_to_string(&mut contents).unwrap();

    let stocks: Vec<HashMap<String, String>> = serde_json::from_str(&contents).unwrap();
    let stock = stocks.first().unwrap();

    println!("{}", strfmt(matches.value_of("format").unwrap(), stock).unwrap());
}
