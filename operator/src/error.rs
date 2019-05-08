//
// Created on Wed May 08 2019
//
// Copyright (c) 2019 Cryptoeconomics Lab, Inc.
// This file is part of Plasma Chamber.
//

// error definition for plasma chain.
#[derive(Debug)]
pub enum Error {
    ParseError(::std::net::AddrParseError),
    IoError(::std::io::Error),
}
