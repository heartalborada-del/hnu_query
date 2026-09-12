use chrono::{DateTime, Utc};
use hnu_query_macros::traced;
use serde::{Deserialize, Serialize};
use crate::iportal::util::deserialize_timestamp;
use crate::iportal::login::IPortalToken;
use crate::utils::obs::{fetch_time, parse_time};

/// 当前账号信息结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountInfo {
    pub uid: String,
    pub name: String,
    pub xgh: String, //学号
    pub identity: String,
    pub identity_id: String,
    pub sex: u8,
    pub depart: String,
    pub mobile: String,
    pub email: String,
    // organ: HashMap<String, u8>,
    pub avatar: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub time: DateTime<Utc>,
    pub is_manager: bool,
    pub is_app_manager: bool,
    pub is_process_manager: bool,
    //user_config: Option<serde_json::Value>,
}

mod fetch;
mod parse;

/// 获取当前账号信息
/// # Arguments
/// * `token` - IPortalToken
/// # Returns
/// 返回当前账号信息，包括用户ID、姓名、学号、身份、性别、部门、手机号、邮箱、头像、时间戳和是否为管理员等信息
#[traced(subsystem = "iportal", skip(token))]
pub async fn get_account_info(
    token: &IPortalToken,
) -> Result<AccountInfo, crate::Error<crate::cas::error::TokenExpired>> {
    let json_str: String = fetch_time!(fetch::fetch_info(token).await)?;
    let info = parse_time!(parse::parse_account_info(&json_str))?;
    Ok(info)
}

#[cfg(test)]
mod tests {
    use crate::{
        iportal::{info::get_account_info, test::get_iportal_token},
        test::TestResult,
    };

    #[tokio::test]
    #[ignore]
    async fn test_fetch_info() -> TestResult<()> {
        let token = get_iportal_token().await?;
        let info = get_account_info(&token).await?;
        println!("{info:#?}");
        Ok(())
    }
}
