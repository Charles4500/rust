use std::env;

#[derive(Clone)]
pub struct MpesaConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub shortcode: String,
    pub passkey: String,
    pub callback_url: String,
    pub base_url: String,
}

impl MpesaConfig {
    pub fn from_env() -> Self {
        Self {
            consumer_key: env::var("CONSUMER_KEY").expect("CONSUMER_KEY missing"),

            consumer_secret: env::var("CONSUMER_SECRET").expect("CONSUMER_SECRET missing"),

            shortcode: env::var("SHORTCODE").expect("SHORTCODE missing"),

            passkey: env::var("PASSKEY").expect("PASSKEY missing"),

            callback_url: env::var("CALLBACK_URL").expect("CALLBACK_URL missing"),

            base_url: env::var("MPESA_BASE_URL")
                .unwrap_or_else(|_| "https://sandbox.safaricom.co.ke".into()),
        }
    }
}
