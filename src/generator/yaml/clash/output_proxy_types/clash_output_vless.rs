use super::CommonProxyOptions;
use crate::models::Proxy;
use crate::utils::{is_empty_option_string, is_u32_option_zero};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Reality options for VLESS proxy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RealityOptions {
    #[serde(rename = "public-key")]
    pub public_key: String,
    #[serde(rename = "short-id")]
    pub short_id: String,
}

/// HTTP options for VLESS proxy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HTTPOptions {
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Vec<String>>>,
}

/// HTTP2 options for VLESS proxy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HTTP2Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Vec<String>>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub path: Option<String>,
}

/// gRPC options for VLESS proxy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GrpcOptions {
    #[serde(
        rename = "grpc-service-name",
        skip_serializing_if = "is_empty_option_string"
    )]
    pub grpc_service_name: Option<String>,
}

/// WebSocket options for VLESS proxy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WSOptions {
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(rename = "max-early-data", skip_serializing_if = "is_u32_option_zero")]
    pub max_early_data: Option<u32>,
    #[serde(
        rename = "early-data-header-name",
        skip_serializing_if = "is_empty_option_string"
    )]
    pub early_data_header_name: Option<String>,
    #[serde(rename = "v2ray-http-upgrade", skip_serializing_if = "Option::is_none")]
    pub v2ray_http_upgrade: Option<bool>,
    #[serde(
        rename = "v2ray-http-upgrade-fast-open",
        skip_serializing_if = "Option::is_none"
    )]
    pub v2ray_http_upgrade_fast_open: Option<bool>,
}

/// VLESS proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct VLessProxy {
    #[serde(flatten)]
    pub common: CommonProxyOptions,
    pub uuid: String,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_addr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xudp: Option<bool>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub packet_encoding: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality_opts: Option<RealityOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_opts: Option<HTTPOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2_opts: Option<HTTP2Options>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_opts: Option<GrpcOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws_opts: Option<WSOptions>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub ws_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws_headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub servername: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub fingerprint: Option<String>,
    #[serde(
        skip_serializing_if = "is_empty_option_string",
        rename = "client-fingerprint"
    )]
    pub client_fingerprint: Option<String>,
}

impl VLessProxy {
    /// Create a new VLESS proxy
    pub fn new(common: CommonProxyOptions) -> Self {
        Self {
            common,
            uuid: String::new(),
            flow: None,
            tls: None,
            alpn: None,
            packet_addr: None,
            xudp: None,
            packet_encoding: None,
            network: None,
            reality_opts: None,
            http_opts: None,
            h2_opts: None,
            grpc_opts: None,
            ws_opts: None,
            ws_path: None,
            ws_headers: None,
            servername: None,
            fingerprint: None,
            client_fingerprint: None,
        }
    }
}

impl From<Proxy> for VLessProxy {
    fn from(proxy: Proxy) -> Self {
        let common =
            CommonProxyOptions::builder(proxy.remark.clone(), proxy.hostname.clone(), proxy.port)
                .udp(proxy.udp)
                .tfo(proxy.tcp_fast_open)
                .skip_cert_verify(proxy.allow_insecure)
                .sni(proxy.sni.clone())
                .build();

        let mut vless = VLessProxy::new(common);

        // 从 combined_proxy 获取 VLESS 特有配置
        if let Some(ref combined) = proxy.combined_proxy {
            if let crate::models::proxy_node::combined::CombinedProxy::Vless(ref vless_proxy) =
                combined
            {
                vless.uuid = vless_proxy.uuid.clone();
                vless.flow = vless_proxy.flow.clone();
                vless.tls = Some(vless_proxy.tls);
                vless.network = vless_proxy.network.clone();
                vless.packet_addr = vless_proxy.packet_addr;
                // vless should force udp to true
                vless.common.udp = Some(vless_proxy.udp);
                vless.xudp = vless_proxy.xudp;
                vless.packet_encoding = vless_proxy.packet_encoding.clone();
                vless.fingerprint = vless_proxy.fingerprint.clone();
                vless.client_fingerprint = vless_proxy.client_fingerprint.clone();

                // 处理 ALPN
                if !vless_proxy.alpn.is_empty() {
                    vless.alpn = Some(vless_proxy.alpn.iter().cloned().collect());
                }

                // 处理 Reality 配置
                if let (Some(public_key), Some(short_id)) = (
                    &vless_proxy.reality_public_key,
                    &vless_proxy.reality_short_id,
                ) {
                    vless.reality_opts = Some(RealityOptions {
                        public_key: public_key.clone(),
                        short_id: short_id.clone(),
                    });
                }

                // 处理不同网络类型的特殊配置
                if let Some(network) = &vless_proxy.network {
                    match network.as_str() {
                        "ws" => {
                            let ws_opts = WSOptions {
                                path: vless_proxy.ws_path.clone(),
                                headers: vless_proxy.ws_headers.clone(),
                                max_early_data: None,
                                early_data_header_name: None,
                                v2ray_http_upgrade: None,
                                v2ray_http_upgrade_fast_open: None,
                            };
                            vless.ws_opts = Some(ws_opts);
                        }
                        "http" => {
                            let http_opts = HTTPOptions {
                                method: vless_proxy.http_method.clone(),
                                path: vless_proxy.http_path.as_ref().map(|p| vec![p.clone()]),
                                headers: vless_proxy.http_headers.clone(),
                            };
                            vless.http_opts = Some(http_opts);
                        }
                        "h2" => {
                            let h2_opts = HTTP2Options {
                                host: vless_proxy.h2_host.clone(),
                                path: vless_proxy.h2_path.clone(),
                            };
                            vless.h2_opts = Some(h2_opts);
                        }
                        "grpc" => {
                            let grpc_opts = GrpcOptions {
                                grpc_service_name: vless_proxy.grpc_service_name.clone(),
                            };
                            vless.grpc_opts = Some(grpc_opts);
                        }
                        _ => {}
                    }
                }

                // 设置 servername
                vless.servername = vless_proxy.servername.clone();
            }
        } else {
            // 如果没有 combined_proxy，则使用默认字段
            vless.uuid = String::new();
            vless.network = proxy.transfer_protocol.clone();

            if let Some(network) = &proxy.transfer_protocol {
                match network.as_str() {
                    "ws" => {
                        let ws_opts = WSOptions {
                            path: None,
                            headers: None,
                            max_early_data: None,
                            early_data_header_name: None,
                            v2ray_http_upgrade: None,
                            v2ray_http_upgrade_fast_open: None,
                        };

                        if let Some(path) = &proxy.path {
                            vless.ws_path = Some(path.clone());
                        }

                        if let Some(host) = &proxy.host {
                            let mut headers = HashMap::new();
                            headers.insert("Host".to_string(), host.clone());
                            vless.ws_headers = Some(headers);
                        }

                        vless.ws_opts = Some(ws_opts);
                    }
                    "grpc" => {
                        let grpc_opts = GrpcOptions {
                            grpc_service_name: None,
                        };

                        if let Some(path) = &proxy.path {
                            vless.grpc_opts = Some(GrpcOptions {
                                grpc_service_name: Some(path.clone()),
                            });
                        } else {
                            vless.grpc_opts = Some(grpc_opts);
                        }
                    }
                    _ => {}
                }
            }
        }

        vless
    }
}
