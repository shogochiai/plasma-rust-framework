extern crate jsonrpc_core;
extern crate jsonrpc_derive;

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait PlasmaRpc {
    /// Returns a protocol version
    #[rpc(name = "protocolVersion")]
    fn protocol_version(&self) -> Result<String>;
}
