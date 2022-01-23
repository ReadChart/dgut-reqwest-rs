use super::errors::GetSessionIDError;
use crate::structs::Criteria;
use reqwest::header;

pub fn common_login(
    criteria: Criteria,
    username: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let (session_id, csrf_token) = response_keyword_extraction(
        criteria.value().unwrap(),
        "PHPSESSID",
        26_usize,
        "token = \"",
        32_usize,
    )?;
    let body = format!(
        "username={}&password={}&__token__={}&wechat_verify=",
        username, password, csrf_token
    );
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(criteria.value().unwrap())
        .header(header::COOKIE, format!("PHPSESSID={}", &session_id))
        .header(
            header::CONTENT_TYPE,
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .header("X-Requested-With", "XMLHttpRequest")
        .body(body)
        .send()?
        .text()?;
    let token_start_idx = resp.find("token").unwrap();
    let token_end_idx = resp.find("&state").unwrap();
    Ok(extract_text(
        &resp,
        token_start_idx + "token=".len(),
        token_end_idx,
    ))
}
pub fn response_keyword_extraction(
    url: &str,
    session_id_keyword: &str,
    session_id_length: usize,
    csrf_token_keyword: &str,
    csrf_token_length: usize,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)
        .map_err(|_| GetSessionIDError::RequestFailure)
        .unwrap();
    let headers = resp.headers().clone();
    let body = resp
        .text()
        .map_err(|_| GetSessionIDError::RequestFailure)
        .unwrap();
    let mut session_id: Option<String> = None;
    for entry in headers.values().into_iter() {
        let entry_str = entry.to_str().unwrap();
        let start_idx = entry_str.find(session_id_keyword);
        session_id = match start_idx {
            None => continue,
            Some(start_idx) => Some(extract_text(
                entry_str,
                start_idx + session_id_keyword.len() + 1,
                start_idx + session_id_keyword.len() + 1 + session_id_length,
            )),
        };
        break;
    }
    let start_idx = body.find(csrf_token_keyword);
    let csrf_token = match start_idx {
        None => None,
        Some(start_idx) => Some(extract_text(
            &body,
            start_idx + csrf_token_keyword.len(),
            start_idx + csrf_token_keyword.len() + csrf_token_length,
        )),
    };
    match (session_id, csrf_token) {
        (Some(session_id), Some(csrf_token)) => Ok((session_id, csrf_token)),
        _ => Err(Box::new(GetSessionIDError::KeywordMatchingFailure)),
    }
}

#[inline]
fn extract_text(value: &str, start_idx: usize, end_idx: usize) -> String {
    String::from(&value[start_idx..end_idx])
}

#[cfg(test)]
mod authentication_tests {
    use crate::authentication::{common_login, response_keyword_extraction};
    use crate::structs::Criteria;

    #[test]
    fn response_keyword_extraction_test() -> Result<(), Box<dyn std::error::Error>> {
        let session_id_keyword = "PHPSESSID";
        let session_id_length = 26_usize;
        let csrf_token_keyword = "token = \"";
        let csrf_token_length = 32_usize;
        let url = Criteria::STU.value().unwrap();
        let (session_id, csrf_token) = response_keyword_extraction(
            url,
            session_id_keyword,
            session_id_length,
            csrf_token_keyword,
            csrf_token_length,
        )?;
        let res = (session_id.len(), csrf_token.len());
        assert_eq!(res, (26_usize, 32_usize));
        Ok(())
    }
    #[test]
    fn common_login_test() -> Result<(), Box<dyn std::error::Error>> {
        let res = common_login(Criteria::YQFK_DAKA, "", "")?;
        assert_eq!(res.len(), 43);
        Ok(())
    }
}
