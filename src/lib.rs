use sauron::prelude::*;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate log;

mod app;
mod utils;

#[wasm_bindgen(start)]
pub fn main() {
  utils::set_panic_hook();
  utils::init_log();
  trace!("Logging initialized.");
  Program::mount_to_body(app::App::new());
}
