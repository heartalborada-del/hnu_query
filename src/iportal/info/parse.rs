use crate::{
    cas::error::TokenExpired,
    error::parse_err,
    iportal::{info::AccountInfo, util::iportal_jsondata_precheck},
};

pub fn parse_account_info(json_str: &str) -> Result<AccountInfo, crate::Error<TokenExpired>> {
    let info = iportal_jsondata_precheck(json_str)?;
    let account_info: AccountInfo = serde_json::from_value(info)
        .map_err(|e| parse_err(format!("JSON解析错误: {}", e).as_str(), json_str))?;
    Ok(account_info)
}
