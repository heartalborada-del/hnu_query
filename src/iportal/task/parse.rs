use crate::{
    error::parse_err,
    iportal::{task::ApplyList, util::iportal_jsondata_precheck},
};

pub fn parse_apply_list(
    json_str: &str,
) -> Result<ApplyList, crate::Error<crate::cas::error::TokenExpired>> {
    let json_value: Result<serde_json::Value, crate::Error<crate::cas::error::TokenExpired>> =
        iportal_jsondata_precheck(json_str);
    let task_list: ApplyList = serde_json::from_value(json_value?.clone())
        .map_err(|e| parse_err(format!("JSON解析错误: {e}").as_str(), json_str))?;
    Ok(task_list)
}
