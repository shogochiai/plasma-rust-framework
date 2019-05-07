use log::error;

use super::error::Error;
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_http_server::ServerBuilder;

pub struct HttpOption {
    threads: usize,
    url: String,
}

impl Default for HttpOption {
    fn default() -> Self {
        Self {
            threads: 3,
            url: "127.0.0.1:8080".to_string(),
        }
    }
}

pub fn start(options: HttpOption) {
    let mut io = IoHandler::new();
    io.add_method("say_hello", |_params: Params| {
        Ok(Value::String("hello".to_string()))
    });

    match options
        .url
        .parse()
        .map_err(|_err| Error::ParseError(_err))
        .and_then(|url| {
            ServerBuilder::new(io)
                .threads(options.threads)
                .start_http(&url)
                .map_err(|_err| Error::IoError(_err))
        }) {
        Ok(server) => server.wait(),
        Err(err) => error!("Error at server.wait: {:?}", err),
    }
}
