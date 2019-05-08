extern crate serde;
extern crate serde_derive;

pub mod error;
pub mod rpc;
pub mod server;

use self::server::start;
use env_logger;
use std::env;

pub fn entry() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    println!("Hello, operator!!");
    start(Default::default());
}
