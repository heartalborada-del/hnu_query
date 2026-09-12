use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use hnu_query_macros::traced;
use serde::{Deserialize, Serialize};

use crate::utils::obs::{fetch_time, parse_time};

mod fetch;
mod parse;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardInfo {
    pub account: String,
    pub balance: f64,
}

/// 记录均以人民币分为单位
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardFlowInfoItem {
    #[serde(rename = "mercname")]
    pub merchant_name: String,
    #[serde(rename = "mercacc")]
    pub merchant_account: String,
    #[serde(rename = "occtime", deserialize_with = "parse_time_fn")]
    pub pay_time: DateTime<Utc>,
    #[serde(rename = "sign_tranamt")]
    pub sign_trans_amount: String,
    #[serde(rename = "tranamt")]
    pub trans_amount: String,
    #[serde(rename = "trancode")]
    pub trans_code: String,
    #[serde(rename = "tranname")]
    pub trans_name: String,
}

/// 校园卡交易明细返回的 JSON 结构
/// row_count 为总条数，pagesize 为每页条数，nextpage 为下一页的页码，data 为交易明细列表
/// 当 nextpage 为 0 时，表示没有下一页
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardDetailsItem {
    #[serde(rename = "nextpage")]
    pub next_page: String,
    #[serde(rename = "pagesize")]
    pub page_size: String,
    #[serde(rename = "rowcount")]
    pub row_count: String,
    #[serde(rename = "total")]
    pub data: Vec<CardFlowInfoItem>,
}

fn parse_time_fn<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, "%Y%m%d%H%M%S").map_err(serde::de::Error::custom)?;
    let timezone = FixedOffset::east_opt(8 * 3600).unwrap();
    let dt = timezone
        .from_local_datetime(&dt)
        .single()
        .ok_or_else(|| serde::de::Error::custom("invalid datetime"))?;
    Ok(dt.to_utc())
}

/// 获取校园卡卡号及余额
/// # Returns
/// 返回校园卡信息，包括卡号和余额
#[traced(subsystem = "iportal", skip(token))]
pub async fn get_card_info(
    token: &crate::iportal::login::IPortalToken,
) -> Result<crate::iportal::card::CardInfo, crate::Error<crate::cas::error::TokenExpired>> {
    let json_str = fetch_time!(fetch::fetch_balance(token).await)?;
    let balance = parse_time!(parse::parse_card_balance(&json_str))?;
    Ok(balance)
}

/// 获取校园卡交易明细
/// # Arguments
/// * `token` - IPortalToken
/// * `start` - 开始时间
/// * `end` - 结束时间
/// * `pagesize` - 每页条数，默认 10
/// * `page` - 页码，默认 1
/// * `account` - 校园卡号
/// # Returns
/// 返回校园卡交易明细，包括交易时间、商户名称、交易金额等
///
/// 其中，交易时间为 UTC 时间
///
/// 当 nextpage 为 0 时，表示没有下一页
#[traced(subsystem = "iportal", skip(token))]
pub async fn get_card_details(
    token: &crate::iportal::login::IPortalToken,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    pagesize: Option<u32>,
    page: Option<u32>,
    account: &str,
) -> Result<crate::iportal::card::CardDetailsItem, crate::Error<crate::cas::error::TokenExpired>> {
    let json_str =
        fetch_time!(fetch::fetch_card_details(token, start, end, pagesize, page, account).await)?;
    let details = parse_time!(parse::parse_card_details(&json_str))?;
    Ok(details)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[ignore]
    async fn test_get_card_info() -> crate::test::TestResult<()> {
        let token = crate::iportal::test::get_iportal_token().await?;
        let info = crate::iportal::card::get_card_info(&token).await?;
        println!("{info:#?}");
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_card_details() -> crate::test::TestResult<()> {
        let token = crate::iportal::test::get_iportal_token().await?;
        let card = crate::iportal::card::get_card_info(&token).await?;
        let info = crate::iportal::card::get_card_details(
            &token,
            chrono::Utc::now() - chrono::Duration::days(30),
            chrono::Utc::now(),
            Some(10),
            Some(1),
            card.account.as_str(),
        )
        .await?;
        println!("{info:#?}");
        Ok(())
    }
}
