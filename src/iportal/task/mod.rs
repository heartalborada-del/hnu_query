use std::any::Any;

use crate::{
    iportal::util::deserialize_timestamp,
    utils::obs::{fetch_time, parse_time},
};
use chrono::{DateTime, Utc};
use hnu_query_macros::traced;
use pdf_extract::Object;
use serde::{Deserialize, Serialize};

mod fetch;
mod parse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyList {
    pub total: i64,
    pub list: Vec<ApplyItem>,
}

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

/// 获取申请列表
/// # Arguments
/// * `token` - IPortalToken
/// * `page` - 页码
/// * `page_size` - 每页数量
/// # Returns
/// 返回申请列表，包括总数和申请项列表
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
