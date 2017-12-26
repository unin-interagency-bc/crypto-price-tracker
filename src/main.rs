extern crate coinnect;

use std::{thread, time};
use std::collections::HashMap;
use std::iter::Iterator;


extern crate num_traits;
use num_traits::cast::ToPrimitive;

use coinnect::types::*;
use coinnect::bitstamp::api::BitstampApi;
use coinnect::bitstamp::credentials::BitstampCreds;
use coinnect::exchange::ExchangeApi;

extern crate postgres;
use postgres::{Connection, TlsMode};

struct CurrencyWithId {
    id: i32,
    pair: Pair
}

impl CurrencyWithId {
    fn new(id: i32, pair: Pair) -> Self {
        CurrencyWithId { id: id, pair: pair }
    }
}

fn main() {

    // connect to bitstamp public API - no credentials required
    let creds = BitstampCreds::new("BitstampClient", "", "", "");
    let mut api = BitstampApi::new(creds).unwrap();

    // currencies to collect prices for
    let mut pair_by_id = HashMap::new();
    pair_by_id.insert(1, Pair::BTC_USD);
    pair_by_id.insert(2, Pair::BCH_USD);
    pair_by_id.insert(3, Pair::ETH_USD);
    pair_by_id.insert(4, Pair::LTC_USD);
    pair_by_id.insert(5, Pair::XRP_USD);

    let currencies = pair_by_id.iter()
        .map(|(&k, &v)| CurrencyWithId::new(k, v))
        .collect::<Vec<CurrencyWithId>>();

    // how often to collect prices?
    let sleep_interval = time::Duration::from_millis(15000);

    //TODO: pass connection details as cmd-line params
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432", TlsMode::None).unwrap();

    // loop forever, collecting prices and storing them
    loop {
        collect_prices(&mut api, &conn, &currencies);
        thread::sleep(sleep_interval);
    }
}

fn collect_prices(api: &mut BitstampApi, conn: &Connection, currencies: &Vec<CurrencyWithId>) {

    // only one call per second allowed
    let one_second = time::Duration::from_millis(1000);

    let sql = "INSERT INTO price_history (currency_id, price) VALUES ($1, $2)";

    currencies.iter().for_each(|ccy| {
        match api.ticker(ccy.pair) {
            Ok(ticker) => {
                match ticker.last_trade_price.to_f64() {
                    Some(price) => {
                        println!("{:?}: {:8.2}", ccy.pair, price);
                        match conn.execute(sql, &[&ccy.id, &price]) {
                            Ok(_) => {},
                            Err(e) => println!("Failed to store {:?}: {:8.2} DBERR: {:?}", ccy.pair, price, e)
                        };
                    },
                    _ => {}
                }
            },
            Err(e) => {
                println!("Failed to get price for {:?}: {:?}", ccy.pair, e);
            }
        };
        thread::sleep(one_second);
    });
    println!();
}





