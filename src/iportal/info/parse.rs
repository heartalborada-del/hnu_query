use crate::{cas::error::TokenExpired, error::parse_err, iportal::info::AccountInfo};

pub fn parse_account_info(json_str: &str) -> Result<AccountInfo, crate::Error<TokenExpired>> {
    let json_value: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| parse_err("JSON解析错误", json_str))?;
    if json_value.get("e").and_then(|e| e.as_i64()) != Some(0) {
        return Err(parse_err("服务器返回值错误", json_str));
    }
    let info = json_value
        .get("d")
        .and_then(|d| d.get("info"))
        .ok_or(parse_err("JSON解析错误", json_str))?;

    let account_info: AccountInfo =
        serde_json::from_value(info.clone()).map_err(|e| parse_err("JSON解析错误", json_str))?;
    Ok(account_info)
}
