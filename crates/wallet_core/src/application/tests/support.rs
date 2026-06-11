use super::super::WalletEngine;
use crate::error::WalletError;
use crate::models::*;
use crate::protocol::rpc::AssetBalanceClient;
use crate::storage::WalletDatabase;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

pub(super) const MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

pub(super) struct EngineFixture {
    _dir: tempfile::TempDir,
    pub(super) engine: WalletEngine,
    pub(super) db_path: PathBuf,
}

pub(super) fn engine_fixture() -> EngineFixture {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("wallet.sqlite");
    let database = WalletDatabase::new(&db_path);
    let engine = WalletEngine::new(database);
    engine.initialize().unwrap();
    EngineFixture {
        _dir: dir,
        engine,
        db_path,
    }
}

pub(super) fn reopened_engine(db_path: &Path) -> WalletEngine {
    let engine = WalletEngine::new(WalletDatabase::new(db_path));
    engine.initialize().unwrap();
    engine
}

pub(super) struct StaticTokenClient {
    symbol: &'static str,
    name: &'static str,
    decimals: u8,
}

impl AssetBalanceClient for StaticTokenClient {
    fn supports_chain(&self, chain: ChainId) -> bool {
        matches!(
            chain,
            ChainId::Ethereum
                | ChainId::Bsc
                | ChainId::Polygon
                | ChainId::Arbitrum
                | ChainId::Optimism
        )
    }

    fn fetch_native_balance(
        &self,
        _chain: ChainId,
        _rpc_url: &str,
        _address: &str,
    ) -> Result<String, WalletError> {
        Ok("0".to_string())
    }

    fn fetch_token_balance(
        &self,
        _chain: ChainId,
        _rpc_url: &str,
        _owner_address: &str,
        _contract_address: &str,
        _decimals: u8,
    ) -> Result<String, WalletError> {
        Ok("0".to_string())
    }

    fn fetch_token_metadata(
        &self,
        chain: ChainId,
        _rpc_url: &str,
        contract_address: &str,
    ) -> Result<TokenMetadata, WalletError> {
        Ok(TokenMetadata {
            chain,
            kind: AssetKind::Erc20,
            contract_address: contract_address.to_string(),
            symbol: self.symbol.to_string(),
            name: self.name.to_string(),
            decimals: self.decimals,
        })
    }
}

pub(super) fn static_token_client(symbol: &'static str) -> StaticTokenClient {
    StaticTokenClient {
        symbol,
        name: symbol,
        decimals: 18,
    }
}

pub(super) struct TestRpcServer {
    pub(super) url: String,
    pub(super) requests: Arc<Mutex<Vec<TestHttpRequest>>>,
}

#[derive(Clone)]
pub(super) struct TestHttpRequest {
    pub(super) path: String,
    pub(super) body: serde_json::Value,
    pub(super) body_text: String,
}

impl TestRpcServer {
    pub(super) fn evm(send_response: serde_json::Value) -> Self {
        Self::evm_with_chain_id("0x1", send_response)
    }

    pub(super) fn evm_with_chain_id(
        chain_id: &'static str,
        send_response: serde_json::Value,
    ) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded_requests = Arc::clone(&requests);
        thread::spawn(move || {
            for stream in listener.incoming().take(12) {
                let mut stream = stream.unwrap();
                let request = read_http_json_request(&mut stream);
                let method = request
                    .body
                    .get("method")
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .to_string();
                let id = request
                    .body
                    .get("id")
                    .cloned()
                    .unwrap_or(serde_json::json!(1));
                recorded_requests.lock().unwrap().push(request);
                let response = match method.as_str() {
                    "eth_chainId" => {
                        serde_json::json!({"jsonrpc":"2.0","id":id,"result":chain_id})
                    }
                    "eth_getTransactionCount" => {
                        serde_json::json!({"jsonrpc":"2.0","id":id,"result":"0x7"})
                    }
                    "eth_getBalance" => {
                        serde_json::json!({"jsonrpc":"2.0","id":id,"result":"0x3635c9adc5dea00000"})
                    }
                    "eth_call" => {
                        serde_json::json!({"jsonrpc":"2.0","id":id,"result":"0x000000000000000000000000000000000000000000000001b1ae4d6e2ef50000"})
                    }
                    "eth_gasPrice" => {
                        serde_json::json!({"jsonrpc":"2.0","id":id,"result":"0x3b9aca00"})
                    }
                    "eth_maxPriorityFeePerGas" => {
                        serde_json::json!({"jsonrpc":"2.0","id":id,"result":"0x3b9aca00"})
                    }
                    "eth_getBlockByNumber" => {
                        serde_json::json!({
                            "jsonrpc":"2.0",
                            "id":id,
                            "result":{"baseFeePerGas":"0x77359400"}
                        })
                    }
                    "eth_estimateGas" => {
                        serde_json::json!({"jsonrpc":"2.0","id":id,"result":"0x5208"})
                    }
                    "eth_sendRawTransaction" => send_response.clone(),
                    _ => serde_json::json!({
                        "jsonrpc":"2.0",
                        "id":id,
                        "error":{"code":-32601,"message":"method not found"}
                    }),
                };
                write_http_json_response(&mut stream, &response);
            }
        });
        Self { url, requests }
    }

    pub(super) fn tron() -> Self {
        Self::tron_with_balances(
            10_000_000,
            "000000000000000000000000000000000000000000000001b1ae4d6e2ef50000",
        )
    }

    pub(super) fn tron_with_balances(native_balance: u64, token_balance: &'static str) -> Self {
        Self::tron_with_balances_and_broadcast_response(
            native_balance,
            token_balance,
            serde_json::json!({"result": true}),
        )
    }

    pub(super) fn tron_with_balances_and_broadcast_response(
        native_balance: u64,
        token_balance: &'static str,
        broadcast_response: serde_json::Value,
    ) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded_requests = Arc::clone(&requests);
        thread::spawn(move || {
            for stream in listener.incoming().take(12) {
                let mut stream = stream.unwrap();
                let request = read_http_json_request(&mut stream);
                let response = match request.path.as_str() {
                    "/wallet/getaccount" => serde_json::json!({
                        "balance": native_balance
                    }),
                    "/wallet/triggerconstantcontract" => serde_json::json!({
                        "result": {"result": true},
                        "constant_result": [token_balance]
                    }),
                    "/wallet/createtransaction" => serde_json::json!({
                        "txID": "tron-native-tx-id",
                        "raw_data": {},
                        "raw_data_hex": "0a020001"
                    }),
                    "/wallet/triggersmartcontract" => serde_json::json!({
                        "transaction": {
                            "txID": "tron-token-tx-id",
                            "raw_data": {},
                            "raw_data_hex": "0a020002"
                        }
                    }),
                    "/wallet/broadcasttransaction" => {
                        if broadcast_response
                            .get("result")
                            .and_then(|value| value.as_bool())
                            == Some(true)
                        {
                            serde_json::json!({
                                "result": true,
                                "txid": request
                                    .body
                                    .get("txID")
                                    .and_then(|value| value.as_str())
                                    .unwrap_or("tron-tx-id")
                            })
                        } else {
                            broadcast_response.clone()
                        }
                    }
                    _ => serde_json::json!({"result": false}),
                };
                recorded_requests.lock().unwrap().push(request);
                write_http_json_response(&mut stream, &response);
            }
        });
        Self { url, requests }
    }

    pub(super) fn btc() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded_requests = Arc::clone(&requests);
        thread::spawn(move || {
            for stream in listener.incoming().take(8) {
                let mut stream = stream.unwrap();
                let request = read_http_json_request(&mut stream);
                let response_json = if request.path == "/fee-estimates" {
                    Some(serde_json::json!({"1": 1.0}))
                } else if request.path.starts_with("/address/") && request.path.ends_with("/utxo") {
                    Some(serde_json::json!([{
                        "txid": "1111111111111111111111111111111111111111111111111111111111111111",
                        "vout": 0,
                        "value": 200000,
                        "status": {"confirmed": true}
                    }]))
                } else {
                    None
                };
                recorded_requests.lock().unwrap().push(request);
                if let Some(response_json) = response_json {
                    write_http_json_response(&mut stream, &response_json);
                } else {
                    write_http_text_response(
                        &mut stream,
                        "2222222222222222222222222222222222222222222222222222222222222222",
                    );
                }
            }
        });
        Self { url, requests }
    }

    pub(super) fn methods(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter_map(|request| request.body.get("method").and_then(|value| value.as_str()))
            .map(str::to_string)
            .collect()
    }

    pub(super) fn request_for_method(&self, method: &str) -> serde_json::Value {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .find(|request| {
                request
                    .body
                    .get("method")
                    .and_then(|value| value.as_str())
                    .is_some_and(|value| value == method)
            })
            .map(|request| request.body.clone())
            .unwrap()
    }

    pub(super) fn paths(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .map(|request| request.path.clone())
            .collect()
    }

    pub(super) fn request_for_path(&self, path: &str) -> serde_json::Value {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .find(|request| request.path == path)
            .map(|request| request.body.clone())
            .unwrap()
    }

    pub(super) fn body_text_for_path(&self, path: &str) -> String {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .find(|request| request.path == path)
            .map(|request| request.body_text.clone())
            .unwrap()
    }
}

fn read_http_json_request(stream: &mut std::net::TcpStream) -> TestHttpRequest {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    let header_end = loop {
        let count = stream.read(&mut chunk).unwrap();
        assert!(count > 0);
        buffer.extend_from_slice(&chunk[..count]);
        if let Some(index) = find_subsequence(&buffer, b"\r\n\r\n") {
            break index + 4;
        }
    };
    let headers = String::from_utf8(buffer[..header_end].to_vec()).unwrap();
    let path = headers
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap()
        .to_string();
    let content_length = headers
        .lines()
        .find_map(|line| line.strip_prefix("Content-Length: "))
        .or_else(|| {
            headers
                .lines()
                .find_map(|line| line.strip_prefix("content-length: "))
        })
        .map(str::trim)
        .unwrap_or("0")
        .parse::<usize>()
        .unwrap();
    while buffer.len() < header_end + content_length {
        let count = stream.read(&mut chunk).unwrap();
        assert!(count > 0);
        buffer.extend_from_slice(&chunk[..count]);
    }
    let body_text =
        String::from_utf8(buffer[header_end..header_end + content_length].to_vec()).unwrap();
    let body = serde_json::from_str(&body_text)
        .unwrap_or_else(|_| serde_json::Value::String(body_text.clone()));
    TestHttpRequest {
        path,
        body,
        body_text,
    }
}

fn write_http_json_response(stream: &mut std::net::TcpStream, body: &serde_json::Value) {
    let body = body.to_string();
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )
    .unwrap();
}

fn write_http_text_response(stream: &mut std::net::TcpStream, body: &str) {
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )
    .unwrap();
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
