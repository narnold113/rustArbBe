extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde_json::Value;

const CC_URL: &str = "https://api.cryptowat.ch/markets/prices";
const HB_URL: &str = "https://api.hitbtc.com/api/2/public/ticker/xrpusdt";

enum Api {
    Cc,
    Hb,
}

fn main() {
    let cc_res = create_json_object(CC_URL).unwrap();
    let hb_res = create_json_object(HB_URL).unwrap();

    let binance_btcusdt: f64 = get_number(Api::Cc, &cc_res, "binance:btcusdt");
    let binance_bccusdt: f64 = get_number(Api::Cc, &cc_res, "binance:bccusdt");
    let binance_ethusdt: f64 = get_number(Api::Cc, &cc_res, "binance:ethusdt");

    let hitbtc_xrp: f64 = get_number(Api::Hb, &hb_res, "last");
}

fn get_api(url: &str) -> String {
    reqwest::get(url)
      .expect("Couldn't make request.")
      .text()
      .expect("Couldn't read response text")
}

fn create_json_object(url: &str) -> Result<Value, ()> {
    let parsed_res: Value = serde_json::from_str(&get_api(url)).unwrap();
    Ok(parsed_res)
}

fn get_number(api: Api, json: &serde_json::Value, info: &str) -> f64 {
    match api {
        Api::Cc => json.get("result").unwrap().get(info).unwrap().as_f64().unwrap(),
        Api::Hb => json.get(info).unwrap().as_str().unwrap().parse::<f64>().unwrap(),
    }
}
