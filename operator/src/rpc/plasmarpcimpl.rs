extern crate jsonrpc_core;

use super::plasmarpc::PlasmaRpc;
use jsonrpc_core::Result;

pub struct PlasmaRpcImpl;

impl PlasmaRpc for PlasmaRpcImpl {
    fn protocol_version(&self) -> Result<String> {
        Ok("0.1.0".into())
    }
}

#[cfg(test)]
mod tests {
    use super::PlasmaRpc;
    use super::PlasmaRpcImpl;
    use jsonrpc_http_server::jsonrpc_core::IoHandler;

    #[test]
    fn test_protocol_version() {
        let mut io = IoHandler::new();

        let rpc = PlasmaRpcImpl;
        io.extend_with(rpc.to_delegate());

        let request = r#"{"jsonrpc": "2.0", "method": "protocolVersion", "params": [], "id": 1}"#;
        let response = r#"{"jsonrpc":"2.0","result":"0.1.0","id":1}"#;

        assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
    }

}
