use crate::{
    cas,
    error::{CheckStatusCodeErr, MapNetworkErr, MapUnexpectedErr},
    iportal::login::IPortalToken,
    utils::client,
};
use reqwest::header::COOKIE;

const INFO_ENDPOINT: &str = "https://iportal.hnu.edu.cn/personal/frontend/data/info";

pub async fn fetch_info(
    token: &IPortalToken,
) -> Result<String, crate::Error<cas::error::TokenExpired>> {
    let cookie = token
        .headers()
        .get(COOKIE)
        .cloned()
        .ok_or_else(|| "iPortal 请求缺少 Cookie".to_string())
        .unexpected_err()?;

    let response = client
        .get(INFO_ENDPOINT)
        .header(COOKIE, cookie)
        .send()
        .await
        .network_err()?
        .status_code_err()
        .await?;

    response.text().await.unexpected_err()
}
