/// Common Authentication URLs
const STU: &str = "https://cas.dgut.edu.cn/home/Oauth/getToken/appid/xgxtt/state/home.html";
const YQFK_DAKA: &str =
    "https://cas.dgut.edu.cn/home/Oauth/getToken/appid/yqfkdaka/state/%2Fhome.html";
const SA: &str = "https://cas.dgut.edu.cn/home/Oauth/getToken/appid/stuaffair/state/home.html";
const JWYD: &str = "https://cas.dgut.edu.cn/home/Oauth/getToken/appid/jwyd/state/home.html";
#[allow(non_camel_case_types)]
pub enum Criteria {
    STU,
    YQFK_DAKA,
    SA,
    JWYD,
}

impl Criteria {
    pub fn value(&self) -> Option<&str> {
        match *self {
            Criteria::STU => Some(STU),
            Criteria::YQFK_DAKA => Some(YQFK_DAKA),
            Criteria::JWYD => Some(JWYD),
            Criteria::SA => Some(SA),
        }
    }
}

#[cfg(test)]
mod struct_tests {
    use crate::structs::{Criteria, STU};
    #[test]
    fn criteria_test() {
        assert_eq!(Criteria::STU.value(), Some(STU));
    }
}
