use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: &str, data: T) -> Self {
        Self {
            success: true,
            message: String::from(message),
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: &str, error: T) -> Self {
        Self {
            success: false,
            message: String::from(message),
            data: None,
            error: Some(error),
        }
    }
}
