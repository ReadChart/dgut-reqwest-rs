use crate::authentication::common_login;
use crate::errors::AuthenticationError;
use crate::structs::Criteria;

#[allow(dead_code)]
mod authentication;
#[allow(dead_code, unreachable_patterns)]
mod errors;
#[allow(dead_code)]
mod structs;

pub fn get_personal_cas_token(criteria: Criteria, username: &str, password: &str) -> Result<String, AuthenticationError> {
    match username.is_empty() || password.is_empty() {
        true => Err(AuthenticationError::EmptyCredentialFailure),
        false => {
            let resp = common_login(criteria, username, password);
            match resp {
                Ok(resp) => Ok(resp),
                _ => Err(AuthenticationError::TokenExtractionFailure)
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[tokio::test]
    async fn connection_test() -> Result<(), Box<dyn std::error::Error>>{
        let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }
    #[test]
    fn common_login_test() -> Result<(), Box<dyn std::error::Error>>{
        // let token = common_login(Criteria::YQFK_DAKA, "", "")?;
        // assert_eq!(43_usize, token.len());
        Ok(())
    }
    #[test]
    fn get_personal_cas_token_test() {
        // let token = get_personal_cas_token(Criteria::YQFK_DAKA, "", "");
        // assert_eq!(token.unwrap().len(), 43_usize)
    }
}
