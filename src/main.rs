extern crate clap;
extern crate curl;
extern crate env_logger;
extern crate serde_json;
extern crate failure;
extern crate log;
extern crate json;
use clap::{App, Arg};
use failure::Error;
use curl::http;
use std::f32;
use serde_json::Value;
use std::env;
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
        eprintln!("{}", err);
        for cause in err.causes().skip(1) {
            eprintln!("Caused by: {}", cause);
        }
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    env_logger::init()?;
    let args: Vec<String> = env::args().collect();
    let matches = App::new("convert")
        .version("0.2.0")
        .author("James McDermott <james.mcdermott89@gmail.com>")
        .about("A command line tool written in rust which converts between currencies.")
        .arg(Arg::with_name("list")
                 .short("l")
                 .long("list")
                 .value_name("list")
                 .takes_value(false)
                 .help("displays all supported currencies.")) 
        .arg(Arg::with_name("rate")
                 .short("r")
                 .long("rate")
                 .value_name("rate")
                 .takes_value(false)
                 .help("displays the rate between two currencies."))
        .arg(Arg::with_name("amount")
                 .required(false)
                 .takes_value(true)
                 .index(1)
                 .requires("amount")
                 .requires("desired")
                 .help("amount to convert from base currency to desired currency."))
        .arg(Arg::with_name("base")
                 .required(false)
                 .takes_value(true)
                 .index(2)
                 .help("base currency, i.e EUR, USD, etc."))
        .arg(Arg::with_name("desired")
                 .required(false)
                 .takes_value(true)
                 .index(3)
                 .multiple(true)
                 .help("desired currency, i.e, EUR, USD, etc"))
        .get_matches();
    if matches.is_present("list") {
        println!("convert supports the following currencies: ");
        let url = "https://api.fixer.io/latest?base=ZAR".to_string(); 
        let response = http::handle()
            .get(url)
            .exec()
            .unwrap();
        let body = std::str::from_utf8(response.get_body()).unwrap();
        let json: Value = serde_json::from_str(body).unwrap_or_else(|e| {
            panic!("Failed to parse json; error is {}", e);
        });
        let obj = json.as_object().and_then(|object| object.get("rates")).and_then(|links| links.as_object()).unwrap_or_else(|| {
            panic!("Failed to get '_links' value from json");
        });
        let mut count = 0;
        for (curr, _val) in obj.iter() {
            if count == 4 {
                println!();
                count = 0;
            }
            print!("    - {}", curr);
            count += 1;
        }
        println!("    - ZAR\n");
    };
    if matches.is_present("rate") {
        let _args: Vec<String> = env::args().collect();
        let (base_rate, desired_rate) = (&args[2], &args[3]);
        let url = "https://api.fixer.io/latest?base=".to_string(); 
        let url = url + base_rate;
        let response = http::handle()
            .get(url)
            .exec()
            .unwrap();
        let body = std::str::from_utf8(response.get_body()).unwrap();
        let json: Value = serde_json::from_str(body).unwrap_or_else(|e| {
            panic!("Failed to parse json; error is {}", e);
        });
        let obj = json.as_object().and_then(|object| object.get("rates")).and_then(|links| links.as_object()).unwrap_or_else(|| {
            panic!("Failed to get '_links' value from json");
        });
        for (curr, val) in obj.iter() {
            if curr == desired_rate {
                println!("The current rate for the {} is {:?}", curr, val);
            }
        }
    }
    else {
        let amount = matches.value_of("amount").unwrap().to_string().parse::<f32>()?;
        let base = matches.value_of("base").unwrap();
        let desired = matches.value_of("desired").unwrap();
        let base_link = "https://api.fixer.io/latest?base=".to_string();
        let curr = &base;
        let url = base_link + curr; 
        let response = http::handle()
            .get(url)
            .exec()
            .unwrap();
        let body = std::str::from_utf8(response.get_body()).unwrap();
        let data = json::parse(body).unwrap_or_else(|e| {
            panic!("Failed to parse json; error is {}", e);
        });
        let rate = data["rates"][desired].to_string().parse::<f32>()?;
        let conversion = Conversion::new(amount, rate);
        println!("{} {} is worth {:.2} {}", conversion.amount, base, conversion.amount * conversion.rate, desired);
    }
    Ok(())
}
