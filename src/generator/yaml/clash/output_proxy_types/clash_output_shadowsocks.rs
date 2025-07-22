use super::CommonProxyOptions;
use crate::models::Proxy;
use crate::utils::is_empty_option_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shadowsocks proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ShadowsocksProxy {
    #[serde(flatten)]
    pub common: CommonProxyOptions,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub cipher: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub plugin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_opts: Option<HashMap<String, String>>,
    // Additional fields from the C++ implementation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_over_tcp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_over_tcp_version: Option<u8>,
    // Fields from the SingBox implementation
    // pub network: Option<String>, // Similar to NetworkList in SingBox
    // pub multiplex: Option<HashMap<String, bool>>, // OutboundMultiplexOptions

    // Fields from the ClashMeta implementation
    // pub client_fingerprint: Option<String>,

    // These fields would be in common options:
    // - udp (already implemented)
    // - tfo (already implemented as tcp_fast_open)
    // - skip_cert_verify (already implemented)
    // - mptcp (not implemented yet)
    // - interface (not implemented yet)
    // - routing_mark (not implemented yet)
    // - ip_version (not implemented yet)
    // - dialer_proxy (not implemented yet)
}

impl ShadowsocksProxy {
    /// Create a new Shadowsocks proxy
    pub fn new(common: CommonProxyOptions) -> Self {
        Self {
            common,
            cipher: None,
            password: None,
            plugin: None,
            plugin_opts: None,
            udp_over_tcp: None,
            udp_over_tcp_version: None,
        }
    }
}

impl From<Proxy> for ShadowsocksProxy {
    fn from(proxy: Proxy) -> Self {
        let common =
            CommonProxyOptions::builder(proxy.remark.clone(), proxy.hostname.clone(), proxy.port)
                .udp(proxy.udp)
                .tfo(proxy.tcp_fast_open)
                .skip_cert_verify(proxy.allow_insecure)
                .sni(proxy.sni.clone())
                .build();

        let mut ss = ShadowsocksProxy::new(common);

        ss.cipher = proxy.encrypt_method;
        ss.password = proxy.password;
        ss.plugin = proxy.plugin;

        if let Some(plugin_opts) = proxy.plugin_option {
            let mut opts = HashMap::new();

            for opt in plugin_opts.split(';') {
                let parts: Vec<&str> = opt.split('=').collect();
                if parts.len() == 2 {
                    opts.insert(parts[0].to_string(), parts[1].to_string());
                }
            }

            ss.plugin_opts = Some(opts);
        }

        // Map combined_proxy fields if available
        if let Some(ref combined) = proxy.combined_proxy {
            if let crate::models::proxy_node::combined::CombinedProxy::Shadowsocks(ref ss_proxy) =
                combined
            {
                ss.udp_over_tcp = ss_proxy.udp_over_tcp;
                ss.udp_over_tcp_version = ss_proxy.udp_over_tcp_version;

                // Add any other fields from the combined proxy here
            }
        }

        ss
    }
}
