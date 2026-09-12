use hnu_query_macros::traced;
use serde::{Deserialize, Deserializer};

use crate::{
    iportal::login::IPortalToken,
    utils::obs::{fetch_time, parse_time},
};

mod fetch;
mod parse;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "key", content = "id")]
pub enum PersonalDataTypeEnum {
    #[serde(rename = "book.bookNum")]
    LibBorrow(String),
    #[serde(rename = "mail.unread")]
    MailUnread(String),
    #[serde(rename = "card.balance")]
    Balance(String),
    #[serde(rename = "statistic.lastLoginTime")]
    LastLoginTime(String),
    #[serde(rename = "net.used")]
    NetUsed(String),
}

impl PersonalDataTypeEnum {
    pub fn into_value(self) -> String {
        match self {
            Self::LibBorrow(value)
            | Self::MailUnread(value)
            | Self::Balance(value)
            | Self::LastLoginTime(value)
            | Self::NetUsed(value) => value,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PersonalDataItem {
    #[serde(deserialize_with = "deserialize_string_or_float")]
    pub value: String,
    pub unit: Option<String>,
    #[serde(alias = "title")]
    pub name: String,
    pub email: Option<String>,
}

fn deserialize_string_or_float<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrFloat {
        String(String),
        Float(f64),
    }

    match StringOrFloat::deserialize(deserializer)? {
        StringOrFloat::String(v) => Ok(v),
        StringOrFloat::Float(v) => Ok(v.to_string()),
    }
}

/// 获取个人信息列表
/// # Returns
/// 返回个人信息列表，包括图书馆借阅信息、未读邮件数、校园卡余额、上次登录时间和校园网流量使用情况所对应的ID。
/// 这些ID可以用于获取对应的个人信息详情。
#[traced(subsystem = "iportal", skip(token))]
pub async fn get_personal_data_lists(
    token: &IPortalToken,
) -> Result<Vec<PersonalDataTypeEnum>, crate::Error<crate::cas::error::TokenExpired>> {
    let json_str = fetch_time!(fetch::fetch_personal_data_query_ids(token).await)?;
    let items = parse_time!(parse::parse_personal_data_query_ids(&json_str))?;
    Ok(items)
}

/// 获取个人信息详情
/// # Arguments
/// * `token` - IPortalToken
/// * `type_enum` - PersonalDataTypeEnum
/// # Returns
/// 返回个人信息详情，包括图书馆借阅信息、未读邮件数、校园卡余额、上次登录时间和校园网流量使用情况。
#[traced(subsystem = "iportal", skip(token))]
pub async fn get_personal_data(
    token: &IPortalToken,
    type_enum: PersonalDataTypeEnum,
) -> Result<PersonalDataItem, crate::Error<crate::cas::error::TokenExpired>> {
    let json_str = fetch_time!(fetch::fetch_personal_data(token, type_enum.clone()).await)?;
    let item = parse_time!(parse::parse_personal_data(&json_str))?;
    Ok(item)
}

#[cfg(test)]
mod tests {

    use crate::{
        iportal::{
            personal::{get_personal_data, get_personal_data_lists},
            test::get_iportal_token,
        },
        test::TestResult,
    };

    #[tokio::test]
    #[ignore]
    async fn test_fetch_personal_data_query_ids() -> TestResult<()> {
        let token = get_iportal_token().await?;
        let ids = get_personal_data_lists(&token).await?;
        println!("{ids:#?}");
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_fetch_personal_data() -> TestResult<()> {
        let token = get_iportal_token().await?;
        let ids = get_personal_data_lists(&token).await?;
        for id in ids {
            let item = get_personal_data(&token, id.clone()).await?;
            println!("{id:?}: {item:#?}");
        }
        Ok(())
    }
}
