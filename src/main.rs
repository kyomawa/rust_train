use serde::Serialize;

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<T>,
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

    fn error(message: &str, error: T) -> ApiResponse<T> {
        ApiResponse {
            success: false,
            message: String::from(message),
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Serialize)]
struct User {
    #[serde(rename = "id")]
    _id: u8,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    age: u8,
}

impl User {
    fn new(first_name: &str, last_name: &str, username: &str, email: &str, age: u8) -> User {
        User {
            _id: 5,
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
    let user2 = User::new("john", "doe", "johnd", "john.doe@gmail.com", 46);
    let v = vec![user, user2];
    let response_success = ApiResponse::success("Users were successfully retrieved.", v);
    let response_error = ApiResponse::error(
        "An error occured while trying to retrieve users.",
        "No user with this id exist",
    );

    println!(
        "{}",
        serde_json::to_string_pretty(&response_success).unwrap()
    );
    println!("{}", serde_json::to_string_pretty(&response_error).unwrap());
}
