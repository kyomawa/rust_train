use serde::Serialize;

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(message: &str, data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            message: String::from(message),
            data: Some(data),
            error: None,
        }
    }
}

impl ApiResponse<String> {
    fn error(message: &str, error: &str) -> ApiResponse<String> {
        ApiResponse {
            success: false,
            message: String::from(message),
            data: None,
            error: Some(String::from(error)),
        }
    }
}

#[derive(Debug, Serialize)]
struct User {
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    age: u8,
}

impl User {
    fn new(first_name: &str, last_name: &str, username: &str, email: &str, age: u8) -> User {
        User {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            username: String::from(username),
            email: String::from(email),
            age,
        }
    }
}

fn main() {
    let user = User::new(
        "bryan",
        "cellier",
        "kyomawa",
        "bryan.cellier.pro@gmail.com",
        23,
    );
    let response_success = ApiResponse::success("User was successfully retrieved.", user);
    let response_error = ApiResponse::error(
        "An error occured during the user retrieving.",
        "No user with this id exist",
    );

    println!(
        "{}",
        serde_json::to_string_pretty(&response_success).unwrap()
    );
    println!("{}", serde_json::to_string_pretty(&response_error).unwrap());
}
