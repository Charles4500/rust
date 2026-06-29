use serde::{Deserialize, Serialize};

use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct StkPushRequest {
    #[validate(length(min = 10, max = 15))]
    pub phone_number: String,

    #[validate(range(min = 1))]
    pub amount: u32,

    #[validate(length(min = 1))]
    pub order_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,

    pub expires_in: String,
}

#[derive(Debug, Serialize)]
pub struct StkPushPayload {
    #[serde(rename = "BusinessShortCode")]
    pub business_shortcode: String,

    #[serde(rename = "Password")]
    pub password: String,

    #[serde(rename = "Timestamp")]
    pub timestamp: String,

    #[serde(rename = "TransactionType")]
    pub transaction_type: String,

    #[serde(rename = "Amount")]
    pub amount: u32,

    #[serde(rename = "PartyA")]
    pub party_a: String,

    #[serde(rename = "PartyB")]
    pub party_b: String,

    #[serde(rename = "PhoneNumber")]
    pub phone_number: String,

    #[serde(rename = "CallBackURL")]
    pub callback_url: String,

    #[serde(rename = "AccountReference")]
    pub account_reference: String,

    #[serde(rename = "TransactionDesc")]
    pub transaction_desc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StkPushResponse {
    #[serde(rename = "MerchantRequestID")]
    pub merchant_request_id: Option<String>,

    #[serde(rename = "CheckoutRequestID")]
    pub checkout_request_id: Option<String>,

    #[serde(rename = "ResponseCode")]
    pub response_code: Option<String>,

    #[serde(rename = "ResponseDescription")]
    pub response_description: Option<String>,

    #[serde(rename = "CustomerMessage")]
    pub customer_message: Option<String>,

    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,

    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}
