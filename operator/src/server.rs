//
// Created on Wed May 08 2019
//
// Copyright (c) 2019 Cryptoeconomics Lab, Inc.
// This file is part of Plasma Chamber.
//

/// json rpc server.
use log::error;

use super::error::Error;
use super::rpc::plasmarpc::PlasmaRpc;
use super::rpc::plasmarpcimpl::PlasmaRpcImpl;
use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;

/// Options for Plasma JSON RPC server.
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

    let rpc = PlasmaRpcImpl;
    io.extend_with(rpc.to_delegate());

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
