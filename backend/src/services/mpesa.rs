use base64::{engine::general_purpose::STANDARD, Engine};

use reqwest::Client;

use crate::{
    configs::mpesa::MpesaConfig,
    models::mpesa::{AccessTokenResponse, StkPushPayload, StkPushRequest, StkPushResponse},
    utils::mpesa::{normalize_phone_number, password, timestamp},
};

#[derive(Clone)]
pub struct MpesaService {
    client: Client,
    config: MpesaConfig,
}

impl MpesaService {
    pub fn new(config: MpesaConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    async fn access_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        let auth = STANDARD.encode(format!(
            "{}:{}",
            self.config.consumer_key, self.config.consumer_secret
        ));

        let response = self
            .client
            .get(format!(
                "{}/oauth/v1/generate?grant_type=client_credentials",
                self.config.base_url
            ))
            .header("Authorization", format!("Basic {}", auth))
            .send()
            .await?;

        let token: AccessTokenResponse = response.json().await?;

        Ok(token.access_token)
    }

    pub async fn stk_push(
        &self,
        request: StkPushRequest,
    ) -> Result<StkPushResponse, Box<dyn std::error::Error>> {
        let token = self.access_token().await?;

        let timestamp = timestamp();

        let password = password(&self.config.shortcode, &self.config.passkey, &timestamp);

        let phone = normalize_phone_number(&request.phone_number);

        let payload = StkPushPayload {
            business_shortcode: self.config.shortcode.clone(),

            password,

            timestamp,

            transaction_type: "CustomerPayBillOnline".to_string(),

            amount: request.amount,

            party_a: phone.clone(),

            party_b: self.config.shortcode.clone(),

            phone_number: phone,

            callback_url: self.config.callback_url.clone(),

            account_reference: request.order_id.clone(),

            transaction_desc: format!("Payment for {}", request.order_id),
        };

        println!("{:#?}", payload);

        let response = self
            .client
            .post(format!(
                "{}/mpesa/stkpush/v1/processrequest",
                self.config.base_url
            ))
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await?;

        let body = response.json::<StkPushResponse>().await?;

        Ok(body)
    }
}
