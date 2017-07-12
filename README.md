# cryptocurrency-tracker
Small command line tool to get cryptocurrency information from coinmarketcap.com

## Usage
```
$ ./cryptocurrency-tracker -c BTC ETH XRP -f "{symbol} \${price_usd} {percent_change_24h}%"
BTC $2251.94 -10.87% | ETH $308.036 -1.25% | XRP $0.266229 -1.45%
```
