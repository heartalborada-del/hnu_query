use crate::{error::parse_err, iportal::util::iportal_jsondata_precheck};

pub fn parse_card_balance(
    json_str: &str,
) -> Result<crate::iportal::card::CardInfo, crate::Error<crate::cas::error::TokenExpired>> {
    let json_value = iportal_jsondata_precheck(json_str);
    let balance = serde_json::from_value(json_value?.clone())
        .map_err(|e| parse_err(format!("JSON解析错误: {}", e).as_str(), json_str))?;
    Ok(balance)
}

pub fn parse_card_details(
    json_str: &str,
) -> Result<crate::iportal::card::CardDetailsItem, crate::Error<crate::cas::error::TokenExpired>> {
    let json_value = iportal_jsondata_precheck(json_str);
    let details = serde_json::from_value(json_value?.clone())
        .map_err(|e| parse_err(format!("JSON解析错误: {}", e).as_str(), json_str))?;
    Ok(details)
}
