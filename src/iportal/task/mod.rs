//! 流程申请记录查询。

use crate::{
    iportal::util::deserialize_timestamp,
    utils::obs::{fetch_time, parse_time},
};
use chrono::{DateTime, Utc};
use hnu_query_macros::traced;
use serde::{Deserialize, Serialize};

mod fetch;
mod parse;

/// 分页的申请记录列表。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyList {
    /// 申请记录总数。
    pub total: i64,
    /// 当前页的申请记录。
    pub list: Vec<ApplyItem>,
}

/// 一条流程申请记录。
///
/// 包含申请所对应的应用、发起人、部门、处理进度、时间以及详情链接等信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyItem {
    id: i64,
    apps_id: i32,
    app_name: String,
    #[serde(rename = "name")]
    creator_name: String,
    #[serde(rename = "creator")]
    creator_id: i32,
    #[serde(rename = "number")]
    creator_account: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    inst_created: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_timestamp")]
    inst_finished: DateTime<Utc>,
    percent: u8,
    #[serde(deserialize_with = "deserialize_timestamp")]
    created: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_timestamp")]
    updated: DateTime<Utc>,
    #[serde(rename = "department_name")]
    department: String,
    department_id: i32,
    department_sn: String,
    form_url_view: String,
    form_mobile_url_view: String,
    process_pic_url: String,
    process_log_url: String,
    custom_status: String,
    del_uid: i32,
    del_time: Option<String>,
    third_id: i32,
    third_app_id: String,
    third_app_name: String,
    third_inst_id: String,
    third_inst_name: String,
    third_name: String,
}

/// 分页获取当前账号发起的全部流程申请。
///
/// # Arguments
///
/// - `token`: 个人门户令牌，可以通过
///   [`IPortalToken::acquire_by_cas_login`](crate::iportal::login::IPortalToken::acquire_by_cas_login)
///   获取
/// - `page`: 页码
/// - `page_size`: 每页记录数
///
/// # Returns
///
/// 返回包含记录总数和当前页记录的 [`ApplyList`]。
///
/// # Errors
///
/// 当令牌失效、网络请求失败或响应无法解析时返回错误。
#[traced(subsystem = "iportal", skip(token))]
pub async fn get_apply_list(
    token: &crate::iportal::login::IPortalToken,
    page: i32,
    page_size: i32,
) -> Result<ApplyList, crate::Error<crate::cas::error::TokenExpired>> {
    let json_str = fetch_time!(fetch::fetch_apply_list(token, page, page_size).await)?;
    let task_list = parse_time!(parse::parse_apply_list(&json_str))?;
    Ok(task_list)
}

#[cfg(test)]
mod tests {
    use crate::{
        iportal::{task::get_apply_list, test::get_iportal_token},
        test::TestResult,
    };

    #[tokio::test]
    #[ignore]
    async fn test_fetch_apply_list() -> TestResult<()> {
        let token = get_iportal_token().await?;
        let task_list = get_apply_list(&token, 1, 10).await?;
        println!("{task_list:#?}");
        Ok(())
    }
}
