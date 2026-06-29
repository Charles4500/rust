use base64::{engine::general_purpose::STANDARD, Engine};

use chrono::Utc;

pub fn normalize_phone_number(phone: &str) -> String {
    if phone.starts_with("254") {
        phone.to_string()
    } else if phone.starts_with('0') {
        format!("254{}", &phone[1..])
    } else if phone.starts_with("+254") {
        phone.trim_start_matches('+').to_string()
    } else {
        phone.to_string()
    }
}

pub fn timestamp() -> String {
    Utc::now().format("%Y%m%d%H%M%S").to_string()
}

pub fn password(shortcode: &str, passkey: &str, timestamp: &str) -> String {
    STANDARD.encode(format!("{}{}{}", shortcode, passkey, timestamp))
}
