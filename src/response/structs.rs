use serde::{Serialize, Deserialize};

/*
 * Structs Definitions
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct MySuccessHttpResponse {
    message: String,
    code: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyErrorHttpResponse {
    message: String,
    code: i64,
    cause: Option<String>,
}

/*
 * Constructor functions
 */

pub fn get_success_response(message: &str, code: i64) -> MySuccessHttpResponse {
    MySuccessHttpResponse {
        message: message.to_string(),
        code,
    }
}

pub fn get_error_response(message: &str, code: i64, cause: Option<&str>) -> MyErrorHttpResponse {
    let m_cause = match cause {
        Some(m) => Some(m.to_string()),
        None => None,
    };
    MyErrorHttpResponse {
        message: message.to_string(),
        code,
        cause: m_cause,
    }
}
