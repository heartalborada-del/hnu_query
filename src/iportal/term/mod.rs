mod fetch;
mod parse;

use hnu_query_macros::traced;
use serde::{Deserialize, Serialize};

use crate::{
    iportal::login::IPortalToken,
    utils::obs::{fetch_time, parse_time},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TermInfo {
    pub start_date: String,
    pub end_date: String,
    #[serde(rename = "dsc")]
    pub description: String,
    pub term: String,
    pub year: String,
    pub week: u16, // maybe i8 enough?
}

/// 获取学期信息
/// # Arguments
/// * `token` - IPortalToken
/// * `timestamp` - 时间戳，单位为秒
/// # Returns
/// 返回学期信息，包括学期开始时间、结束时间、学期描述、学期、学年和周数
#[traced(subsystem = "iportal", skip(token))]
pub async fn get_term_info(
    token: &IPortalToken,
    timestamp: i64,
) -> Result<TermInfo, crate::Error<crate::cas::error::TokenExpired>> {
    let json_str = fetch_time!(fetch::fetch_term_info(token, timestamp).await)?;
    let info = parse_time!(parse::parse_term_info(&json_str))?;
    Ok(info)
}

#[cfg(test)]
mod tests {
    use std::time::{self, UNIX_EPOCH};

    use crate::{
        iportal::{term::get_term_info, test::get_iportal_token},
        test::TestResult,
    };

    #[tokio::test]
    #[ignore]
    async fn test_fetch_term_info() -> TestResult<()> {
        let token = get_iportal_token().await?;
        let info = get_term_info(
            &token,
            time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        )
        .await?;
        println!("{info:#?}");
        Ok(())
    }
}
