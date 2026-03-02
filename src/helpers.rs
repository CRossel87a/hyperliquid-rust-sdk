use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use log::info;
use uuid::Uuid;

use crate::consts::*;

fn now_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_millis() as u64
}

pub(crate) fn next_nonce() -> u64 {
    let nonce = CUR_NONCE.fetch_add(1, Ordering::Relaxed);
    let now_ms = now_timestamp_ms();
    if nonce > now_ms + 1000 {
        info!("nonce progressed too far ahead {nonce} {now_ms}");
    }
    // more than 300 seconds behind
    if nonce + 300000 < now_ms {
        CUR_NONCE.fetch_max(now_ms + 1, Ordering::Relaxed);
        return now_ms;
    }
    nonce
}

pub(crate) const WIRE_DECIMALS: u8 = 8;

pub(crate) fn float_to_string_for_hashing(x: f64) -> String {
    let mut x = format!("{:.*}", WIRE_DECIMALS.into(), x);
    while x.ends_with('0') {
        x.pop();
    }
    if x.ends_with('.') {
        x.pop();
    }
    if x == "-0" {
        "0".to_string()
    } else {
        x
    }
}

pub(crate) fn uuid_to_hex_string(uuid: Uuid) -> String {
    let hex_string = uuid
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<String>>()
        .join("");
    format!("0x{hex_string}")
}

#[derive(Copy, Clone)]
pub enum BaseUrl {
    Localhost,
    Testnet,
    Mainnet,
}

impl BaseUrl {
    pub(crate) fn get_url(&self) -> String {
        match self {
            BaseUrl::Localhost => LOCAL_API_URL.to_string(),
            BaseUrl::Mainnet => MAINNET_API_URL.to_string(),
            BaseUrl::Testnet => TESTNET_API_URL.to_string(),
        }
    }
}

static CUR_NONCE: AtomicU64 = AtomicU64::new(0);

fn init_nonce() {
    CUR_NONCE.fetch_max(now_timestamp_ms(), Ordering::Relaxed);
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(windows, link_section = ".CRT$XCU")]
static INIT: fn() = init_nonce;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_to_string_for_hashing_test() {
        assert_eq!(float_to_string_for_hashing(0.), "0".to_string());
        assert_eq!(float_to_string_for_hashing(-0.), "0".to_string());
        assert_eq!(float_to_string_for_hashing(-0.0000), "0".to_string());
        assert_eq!(
            float_to_string_for_hashing(0.00076000),
            "0.00076".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(0.00000001),
            "0.00000001".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(0.12345678),
            "0.12345678".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(87654321.12345678),
            "87654321.12345678".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(987654321.00000000),
            "987654321".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(87654321.1234),
            "87654321.1234".to_string()
        );
        assert_eq!(float_to_string_for_hashing(0.000760), "0.00076".to_string());
        assert_eq!(float_to_string_for_hashing(0.00076), "0.00076".to_string());
        assert_eq!(
            float_to_string_for_hashing(987654321.0),
            "987654321".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(987654321.),
            "987654321".to_string()
        );
    }
}
