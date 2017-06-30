# rust-crypto-tracker
Small command line tool to track crypto stocks

## rust compile
```
cargo build
```

## rust launcher
```
$ ./rust-crypto-tracker -h
rust-crypto-tracker 0.1.0
Joe Roberts <joe@resin.io>
Get crypto stock information from coinmarketcap.com

USAGE:
    rust-crypto-tracker [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --crypto <crypto>    Enter the crypto [default: ethereum]
    -f, --format <format>    Enter the format [default: {name}, {symbol}, {rank}, {price_usd}, {price_btc},
                             {24h_volume_usd}, {market_cap_usd}, {available_supply},
                             {total_supply}, {percent_change_1h}, {percent_change_24h},
                             {percent_change_7d}, {last_updated}]
```

## neon compile
```
npm install
```

## neon launcher
```
$ npm start
```
