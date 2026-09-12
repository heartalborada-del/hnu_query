use crate::iportal::term::TermInfo;

pub fn parse_term_info(
    json_str: &str,
) -> Result<TermInfo, crate::Error<crate::cas::error::TokenExpired>> {
    let info = crate::iportal::util::iportal_jsondata_precheck(json_str)?;
    let term_info: TermInfo = serde_json::from_value(info)
        .map_err(|e| crate::error::parse_err(format!("JSON解析错误: {}", e).as_str(), json_str))?;
    Ok(term_info)
}
