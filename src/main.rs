extern crate clap;
extern crate curl;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;
extern crate json;
#[macro_use]

use clap::{App, Arg};
use failure::Error;
use curl::http;
use json::parse;
use std::f32;

#[derive(Debug)]
struct Conversion {
    amount: f32,
    rate: f32,
}

impl Conversion {
    fn new(amount: f32, rate: f32) -> Self {
        Conversion {
            amount: amount,
            rate: rate,
        }
    }
}


fn main() {
    use std::process::exit;

    if let Err(err) = run() {
        debug!("{:?}", err);
        eprintln!("{}", err);
        for cause in err.causes().skip(1) {
            eprintln!("Caused by: {}", cause);
        }
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    env_logger::init()?; 
    let matches = App::new("plutus")
        .version("0.1.0")
        .author("James McDermott <james.mcdermott89@gmail.com>")
        .about("A command line tool written in rust which converts between currencies.")
        .arg(Arg::with_name("amount")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("amount to convert from base currency to desired currency."))
        .arg(Arg::with_name("base")
                 .required(true)
                 .takes_value(true)
                 .index(2)
                 .help("base currency, i.e EUR, USD, etc."))
        .arg(Arg::with_name("desired")
                 .required(true)
                 .takes_value(true)
                 .index(3)
                 .help("desired currency, i.e, EUR, USD, etc"))
        .get_matches();
    
    let amount = matches.value_of("amount").unwrap().to_string().parse::<f32>()?;
    let base = matches.value_of("base").unwrap();
    let desired = matches.value_of("desired").unwrap();
    let base_link = "https://api.fixer.io/latest?base=".to_string();
    let curr = &base;
    let url = base_link + &curr;
    
    let response = http::handle()
        .get(url)
        .exec()
        .unwrap();
    let body = std::str::from_utf8(response.get_body()).unwrap();
    let data = json::parse(body).unwrap();
    let rate = data["rates"][desired].to_string().parse::<f32>()?;
    let conversion = Conversion::new(amount, rate);
    println!("{} {} is worth {:.2} {}", conversion.amount, base, conversion.amount * conversion.rate, desired);
    Ok(())
}
