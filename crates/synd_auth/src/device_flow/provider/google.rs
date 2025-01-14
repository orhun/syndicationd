use std::borrow::Cow;

use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{
    config,
    device_flow::{DeviceAuthorizationRequest, Provider},
};

#[derive(Clone)]
pub struct Google {
    pub client_id: Cow<'static, str>,
    pub client_secret: Cow<'static, str>,
}

impl Default for Google {
    fn default() -> Self {
        Self::new(config::google::CLIENT_ID, config::google::CLIENT_ID2)
    }
}

impl Google {
    const DEVICE_AUTHORIZATION_ENDPOINT: &'static str = "https://oauth2.googleapis.com/device/code";
    const TOKEN_ENDPOINT: &'static str = "https://oauth2.googleapis.com/token";
    /// <https://developers.google.com/identity/gsi/web/guides/devices#obtain_an_id_token_and_refresh_token>
    const GRANT_TYPE: &'static str = "http://oauth.net/grant_type/device/1.0";

    ///  <https://developers.google.com/identity/gsi/web/guides/devices#obtain_a_user_code_and_verification_url>
    const SCOPE: &'static str = "email";

    pub fn new(
        client_id: impl Into<Cow<'static, str>>,
        client_secret: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeviceAccessTokenRequest<'s> {
    grant_type: Cow<'static, str>,
    pub code: Cow<'s, str>,
    pub client_id: Cow<'s, str>,
    pub client_secret: Cow<'s, str>,
}

impl Provider for Google {
    type DeviceAccessTokenRequest<'d> = DeviceAccessTokenRequest<'d>;

    fn device_authorization_endpoint(&self) -> Url {
        Url::parse(Self::DEVICE_AUTHORIZATION_ENDPOINT).unwrap()
    }

    fn token_endpoint(&self) -> reqwest::Url {
        Url::parse(Self::TOKEN_ENDPOINT).unwrap()
    }

    fn device_authorization_request(&self) -> DeviceAuthorizationRequest {
        DeviceAuthorizationRequest {
            client_id: self.client_id.clone(),
            scope: Self::SCOPE.into(),
        }
    }

    fn device_access_token_request<'d, 'p: 'd>(
        &'p self,
        device_code: &'d str,
    ) -> DeviceAccessTokenRequest<'d> {
        DeviceAccessTokenRequest {
            grant_type: Self::GRANT_TYPE.into(),
            code: device_code.into(),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
        }
    }
}
