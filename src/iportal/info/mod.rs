use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde::de::Deserializer;
use chrono::TimeZone;

/// 当前账号信息结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountInfo {
    uid: String,
    name: String,
    xgh: String, //学号
    identity: String,
    identity_id: String,
    sex: u8,
    depart: String,
    mobile: String,
    email: String,
    // organ: HashMap<String, u8>,
    #[serde(rename = "avatar")]
    avatar: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    time: DateTime<Utc>,
    is_manager: bool,
    is_app_manager: bool,
    is_process_manager: bool,
    //user_config: Option<serde_json::Value>,
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
        .map_err(serde::de::Error::custom)?;
    let timezone = FixedOffset::east_opt(8 * 3600).unwrap();
    let dt = timezone
        .from_local_datetime(&dt)
        .single()
        .ok_or_else(|| serde::de::Error::custom("invalid datetime"))?;
    Ok(dt.to_utc())
}

pub mod fetch;
pub mod parse;

#[cfg(test)]
mod tests {
    use crate::{
        iportal::{info::{fetch::fetch_info, parse::parse_account_info}, test::get_iportal_token},
        test::TestResult,
    };

    #[tokio::test]
    #[ignore]
    async fn test_fetch_info() -> TestResult<()> {
        let token = get_iportal_token().await?;
        let json = fetch_info(&token).await?;
        let info = parse_account_info(&json)?;
        println!("{info:#?}");
        Ok(())
    }
}
