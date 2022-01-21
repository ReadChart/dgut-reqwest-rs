use crate::authentication::common_login;
use crate::errors::AuthenticationError;
use crate::structs::Criteria;

#[allow(dead_code)]
mod authentication;
#[allow(dead_code, unreachable_patterns)]
mod errors;
#[allow(dead_code)]
mod structs;

pub async fn get_personal_cas_token(criteria: Criteria, username: &str, password: &str) -> Result<String, AuthenticationError> {
    let resp = common_login(criteria, username, password).await;
    match resp {
        Ok(resp) => Ok(resp),
        _ => Err(AuthenticationError::TokenExtractionFailure)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::authentication::common_login;
    use crate::get_personal_cas_token;
    use crate::structs::Criteria;

    #[tokio::test]
    async fn connection_test() -> Result<(), Box<dyn std::error::Error>>{
        let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn common_login_test() -> Result<(), Box<dyn std::error::Error>>{
        let token = common_login(Criteria::YQFK_DAKA, "", "").await?;
        assert_eq!(43_usize, token.len());
        Ok(())
    }
    #[tokio::test]
    async fn get_personal_cas_token_test() {
        let token = get_personal_cas_token(Criteria::YQFK_DAKA, "", "").await;
        assert_eq!(token.unwrap().len(), 43_usize)
    }
}
