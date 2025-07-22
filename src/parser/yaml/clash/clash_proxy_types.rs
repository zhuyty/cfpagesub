use serde::Deserialize;

use super::input_proxy_types::{
    clash_input_anytls::ClashInputAnyTLS, clash_input_http::ClashInputHttp,
    clash_input_hysteria::ClashInputHysteria, clash_input_hysteria2::ClashInputHysteria2,
    clash_input_shadowsocks::ClashInputShadowsocks,
    clash_input_shadowsocksr::ClashInputShadowsocksR, clash_input_snell::ClashInputSnell,
    clash_input_socks5::ClashInputSocks5, clash_input_trojan::ClashInputTrojan,
    clash_input_vless::ClashInputVLess, clash_input_vmess::ClashInputVMess,
    clash_input_wireguard::ClashInputWireGuard,
};

/// Represents a single proxy in Clash configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ClashProxyYamlInput {
    #[serde(rename = "ss")]
    Shadowsocks(ClashInputShadowsocks),

    #[serde(rename = "ssr")]
    ShadowsocksR(ClashInputShadowsocksR),

    #[serde(rename = "vmess")]
    VMess(ClashInputVMess),

    #[serde(rename = "trojan")]
    Trojan(ClashInputTrojan),

    #[serde(rename = "http")]
    Http(ClashInputHttp),

    #[serde(rename = "socks5")]
    Socks5(ClashInputSocks5),

    #[serde(rename = "snell")]
    Snell(ClashInputSnell),

    #[serde(rename = "wireguard")]
    WireGuard(ClashInputWireGuard),

    #[serde(rename = "hysteria")]
    Hysteria(ClashInputHysteria),

    #[serde(rename = "hysteria2")]
    Hysteria2(ClashInputHysteria2),

    #[serde(rename = "vless")]
    VLess(ClashInputVLess),

    #[serde(rename = "anytls")]
    AnyTls(ClashInputAnyTLS),

    // Handle other unknown proxy types
    #[serde(other)]
    Unknown,
}
