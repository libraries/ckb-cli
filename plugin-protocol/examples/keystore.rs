use ckb_types::{h160, H160};

use ckb_cli_plugin_protocol::{
    JsonrpcError, JsonrpcRequest, JsonrpcResponse, KeyStoreRequest, PluginConfig, PluginRequest,
    PluginResponse, PluginRole,
};
use ckb_sdk::rpc::JsonBytes;
use std::convert::TryInto;
use std::io::{self, Write};

fn main() {
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(_n) => {
                let jsonrpc_request: JsonrpcRequest = serde_json::from_str(&line).unwrap();
                let (id, request) = jsonrpc_request.try_into().unwrap();
                if let Some(response) = handle(request) {
                    let jsonrpc_response = JsonrpcResponse::from((id, response));
                    let response_string =
                        format!("{}\n", serde_json::to_string(&jsonrpc_response).unwrap());
                    io::stdout().write_all(response_string.as_bytes()).unwrap();
                    io::stdout().flush().unwrap();
                }
            }
            Err(_err) => {}
        }
    }
}

fn handle(request: PluginRequest) -> Option<PluginResponse> {
    match request {
        PluginRequest::Quit => None,
        PluginRequest::GetConfig => {
            let config = PluginConfig {
                name: String::from("demo_keystore"),
                description: String::from("It's a keystore for demo"),
                daemon: true,
                roles: vec![PluginRole::KeyStore {
                    require_password: true,
                }],
            };
            Some(PluginResponse::PluginConfig(config))
        }
        PluginRequest::KeyStore(keystore_request) => {
            let response = match keystore_request {
                KeyStoreRequest::ListAccount => {
                    let req = PluginRequest::ReadPassword("Your Password:".to_string());
                    let jsonrpc_request = JsonrpcRequest::from((0, req));
                    let request_string =
                        format!("{}\n", serde_json::to_string(&jsonrpc_request).unwrap());
                    // Write message to stderr for plugin debugging
                    eprintln!("Request: {}", request_string);
                    io::stdout().write_all(request_string.as_bytes()).unwrap();
                    io::stdout().flush().unwrap();
                    let mut line = String::new();
                    io::stdin().read_line(&mut line).unwrap();
                    let jsonrpc_response: JsonrpcResponse = serde_json::from_str(&line).unwrap();
                    let (_id, response) = jsonrpc_response.try_into().unwrap();
                    if let PluginResponse::String(password) = response {
                        if password == "bad" {
                            return Some(PluginResponse::Error(JsonrpcError {
                                code: 0,
                                message: String::from("Error password"),
                                data: None,
                            }));
                        }
                    }

                    let accounts = vec![
                        h160!("0xe22f7f385830a75e50ab7fc5fd4c35b134f1e84b"),
                        h160!("0x13e41d6F9292555916f17B4882a5477C01270142"),
                        h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64"),
                    ];
                    PluginResponse::H160Vec(accounts)
                }
                KeyStoreRequest::CreateAccount(_) => {
                    PluginResponse::H160(h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64"))
                }
                KeyStoreRequest::UpdatePassword { .. } => PluginResponse::Ok,
                KeyStoreRequest::Import { .. } => {
                    PluginResponse::H160(h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64"))
                }
                KeyStoreRequest::Export { .. } => PluginResponse::MasterPrivateKey {
                    privkey: JsonBytes::from_vec(vec![3u8; 32]),
                    chain_code: JsonBytes::from_vec(vec![4u8; 32]),
                },
                KeyStoreRequest::Sign { recoverable, .. } => {
                    let signature = if recoverable {
                        vec![1u8; 65]
                    } else {
                        vec![2u8; 64]
                    };
                    PluginResponse::Bytes(JsonBytes::from_vec(signature))
                }
                KeyStoreRequest::ExtendedPubkey { .. } => {
                    PluginResponse::Bytes(JsonBytes::from_vec(vec![
                        0x02, 0x53, 0x1f, 0xe6, 0x06, 0x81, 0x34, 0x50, 0x3d, 0x27, 0x23, 0x13,
                        0x32, 0x27, 0xc8, 0x67, 0xac, 0x8f, 0xa6, 0xc8, 0x3c, 0x53, 0x7e, 0x9a,
                        0x44, 0xc3, 0xc5, 0xbd, 0xbd, 0xcb, 0x1f, 0xe3, 0x37,
                    ]))
                }
                KeyStoreRequest::DerivedKeySet { .. } => PluginResponse::DerivedKeySet {
                    external: vec![
                        (
                            "m/44'/309'/0'/0/19".to_owned(),
                            h160!("0x13e41d6F9292555916f17B4882a5477C01270142"),
                        ),
                        (
                            "m/44'/309'/0'/0/20".to_owned(),
                            h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64"),
                        ),
                    ],
                    change: vec![
                        (
                            "m/44'/309'/0'/1/19".to_owned(),
                            h160!("0x13e41d6F9292555916f17B4882a5477C01270142"),
                        ),
                        (
                            "m/44'/309'/0'/1/20".to_owned(),
                            h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64"),
                        ),
                    ],
                },
                KeyStoreRequest::DerivedKeySetByIndex { .. } => PluginResponse::DerivedKeySet {
                    external: vec![
                        (
                            "m/44'/309'/0'/0/19".to_owned(),
                            h160!("0x13e41d6F9292555916f17B4882a5477C01270142"),
                        ),
                        (
                            "m/44'/309'/0'/0/20".to_owned(),
                            h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64"),
                        ),
                    ],
                    change: vec![
                        (
                            "m/44'/309'/0'/1/19".to_owned(),
                            h160!("0x13e41d6F9292555916f17B4882a5477C01270142"),
                        ),
                        (
                            "m/44'/309'/0'/1/20".to_owned(),
                            h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64"),
                        ),
                    ],
                },
                _ => {
                    return Some(PluginResponse::Error(JsonrpcError {
                        code: 0,
                        message: String::from("Not supported yet"),
                        data: None,
                    }));
                }
            };
            Some(response)
        }
        _ => Some(PluginResponse::Error(JsonrpcError {
            code: 0,
            message: String::from("Invalid request to keystore"),
            data: None,
        })),
    }
}
