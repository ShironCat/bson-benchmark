use js_sys::Date;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate log;

mod utils;

#[derive(Deserialize, Serialize)]
struct Test {
  text: String,
  number: i32,
  vector: Vec<i32>,
}

#[wasm_bindgen(start)]
pub fn main() {
  utils::set_panic_hook();
  utils::init_log();
  let test = Test {
    text: "Lorem ipsum".to_owned(),
    number: 15,
    vector: vec![1, 2, 3, 4],
  };
  for i in 1..5 {
    info!("Run #{}:", i);
    let start = Date::now();
    for _ in 0..500_000 {
      serde_json::from_str::<Test>(serde_json::to_string(&test).unwrap().as_str()).unwrap();
    }
    let stop = Date::now();
    debug!("JSON: {} ms", (stop - start));
    let start = Date::now();
    for _ in 0..500_000 {
      bson::from_bson::<Test>(bson::to_bson(&test).unwrap()).unwrap();
    }
    let stop = Date::now();
    debug!("BSON: {} ms", (stop - start));
  }
}
