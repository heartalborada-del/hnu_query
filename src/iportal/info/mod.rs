//! 当前登录账号信息查询。

use crate::iportal::login::IPortalToken;
use crate::iportal::util::deserialize_timestamp;
use crate::utils::obs::{fetch_time, parse_time};
use chrono::{DateTime, Utc};
use hnu_query_macros::traced;
use serde::{Deserialize, Serialize};

/// 当前登录账号的信息。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountInfo {
    /// 用户 ID。
    pub uid: String,
    /// 姓名。
    pub name: String,
    /// 学工号。
    pub xgh: String,
    /// 当前身份名称。
    pub identity: String,
    /// 当前身份 ID。
    pub identity_id: String,
    /// 性别代码。
    pub sex: u8,
    /// 所属部门。
    pub depart: String,
    /// 手机号码。
    pub mobile: String,
    /// 电子邮箱。
    pub email: String,
    // organ: HashMap<String, u8>,
    /// 头像地址。
    pub avatar: String,
    /// 服务端返回的账号时间戳。
    ///
    /// iPortal 返回的北京时间会转换为 UTC。
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub time: DateTime<Utc>,
    /// 是否为系统管理员。
    pub is_manager: bool,
    /// 是否为应用管理员。
    pub is_app_manager: bool,
    /// 是否为流程管理员。
    pub is_process_manager: bool,
    //user_config: Option<serde_json::Value>,
}

mod fetch;
mod parse;

/// 获取当前登录账号的信息。
///
/// # Arguments
///
/// - `token`: 个人门户令牌，可以通过 [`IPortalToken::acquire_by_cas_login`] 获取
///
/// # Returns
///
/// 返回包含用户、身份、联系方式及管理员状态的 [`AccountInfo`]。
///
/// # Errors
///
/// 当令牌失效、网络请求失败或响应无法解析时返回错误。
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
