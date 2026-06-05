use crate::{handle_command_json, WalletResponse};
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

pub(super) fn response(command_json: &str) -> WalletResponse {
    handle_command_json(command_json)
}

pub(super) struct TestRpcServer {
    pub(super) url: String,
    requests: Arc<Mutex<Vec<Value>>>,
}

impl TestRpcServer {
    pub(super) fn evm() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded_requests = Arc::clone(&requests);
        thread::spawn(move || {
            for stream in listener.incoming().take(8) {
                let mut stream = stream.unwrap();
                let request = read_http_json_body(&mut stream);
                let method = request
                    .get("method")
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .to_string();
                let id = request.get("id").cloned().unwrap_or(json!(1));
                recorded_requests.lock().unwrap().push(request);
                let response = match method.as_str() {
                    "eth_chainId" => json!({"jsonrpc":"2.0","id":id,"result":"0x1"}),
                    "eth_getTransactionCount" => {
                        json!({"jsonrpc":"2.0","id":id,"result":"0x0"})
                    }
                    "eth_getBalance" => {
                        json!({"jsonrpc":"2.0","id":id,"result":"0x3635c9adc5dea00000"})
                    }
                    "eth_gasPrice" => json!({"jsonrpc":"2.0","id":id,"result":"0x3b9aca00"}),
                    "eth_maxPriorityFeePerGas" => {
                        json!({"jsonrpc":"2.0","id":id,"result":"0x3b9aca00"})
                    }
                    "eth_getBlockByNumber" => json!({
                        "jsonrpc":"2.0",
                        "id":id,
                        "result":{"baseFeePerGas":"0x77359400"}
                    }),
                    "eth_estimateGas" => json!({"jsonrpc":"2.0","id":id,"result":"0x5208"}),
                    "eth_sendRawTransaction" => {
                        json!({"jsonrpc":"2.0","id":id,"result":"0xffibroadcast"})
                    }
                    _ => json!({
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

    pub(super) fn saw_method(&self, method: &str) -> bool {
        self.requests.lock().unwrap().iter().any(|request| {
            request
                .get("method")
                .and_then(|value| value.as_str())
                .is_some_and(|value| value == method)
        })
    }
}

fn read_http_json_body(stream: &mut std::net::TcpStream) -> Value {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    let header_end = loop {
        let count = stream.read(&mut chunk).unwrap();
        assert!(count > 0);
        buffer.extend_from_slice(&chunk[..count]);
        if let Some(index) = buffer.windows(4).position(|window| window == b"\r\n\r\n") {
            break index + 4;
        }
    };
    let headers = String::from_utf8(buffer[..header_end].to_vec()).unwrap();
    let content_length = headers
        .lines()
        .find_map(|line| line.strip_prefix("Content-Length: "))
        .or_else(|| {
            headers
                .lines()
                .find_map(|line| line.strip_prefix("content-length: "))
        })
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    while buffer.len() < header_end + content_length {
        let count = stream.read(&mut chunk).unwrap();
        assert!(count > 0);
        buffer.extend_from_slice(&chunk[..count]);
    }
    serde_json::from_slice(&buffer[header_end..header_end + content_length]).unwrap()
}

fn write_http_json_response(stream: &mut std::net::TcpStream, body: &Value) {
    let body = body.to_string();
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )
    .unwrap();
}
