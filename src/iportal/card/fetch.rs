use chrono::{DateTime, Utc};

use crate::{
    error::MapUnexpectedErr,
    iportal::{login::IPortalToken, util::IPortalRequestBuilderExt},
    utils::client,
};

pub async fn fetch_balance(
    token: &IPortalToken,
) -> Result<String, crate::Error<crate::cas::error::TokenExpired>> {
    let url = "https://iportal.hnu.edu.cn/hnu/frontend/user/card-balance";
    let response = client.get(url).send_with_token(token).await?;

    response.text().await.unexpected_err()
}

/// 获取校园卡交易明细
/// account 为校园卡账号，start 和 end 为查询的起止时间，pagesize 为每页条数，page 为页码
/// account 请从 fetch_balance 返回的 JSON 中获取，start 和 end 的格式为 "YYYY-MM-DD"，pagesize 和 page 为可选参数，默认值分别为 10 和 1
pub async fn fetch_card_details(
    token: &IPortalToken,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    pagesize: Option<u32>,
    page: Option<u32>,
    account: &str,
) -> Result<String, crate::Error<crate::cas::error::TokenExpired>> {
    let url = format!(
        "https://iportal.hnu.edu.cn/hnu/frontend/user/card-details?query_start={}&query_end={}&page_size={}&page={}&account={}",
        start.format("%Y-%m-%d"),
        end.format("%Y-%m-%d"),
        pagesize.unwrap_or(10),
        page.unwrap_or(1),
        account
    );
    let response = client.get(url).send_with_token(token).await?;

    response.text().await.unexpected_err()
}
