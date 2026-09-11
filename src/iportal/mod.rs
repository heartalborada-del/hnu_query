pub mod info;
pub mod login;

#[cfg(test)]
mod test;

#[cfg(test)]
mod tests {
    use crate::{iportal::test::get_iportal_token, test::TestResult};

    #[tokio::test]
    #[ignore]
    async fn test_login() -> TestResult<()> {
        let token = get_iportal_token().await?;
        println!("{token:#?}");
        Ok(())
    }
}
