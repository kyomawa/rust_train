#[derive(Debug)]
enum ApiResponseVariant<T> {
    Success(T),
    Error(String),
}

#[derive(Debug)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    variant: ApiResponseVariant<T>,
}

impl<T> ApiResponse<T> {
    fn success(message: &str, data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            message: String::from(message),
            variant: ApiResponseVariant::Success(data),
        }
    }
}

impl ApiResponse<String> {
    fn error(message: &str, error: &str) -> ApiResponse<String> {
        ApiResponse {
            success: false,
            message: String::from(message),
            variant: ApiResponseVariant::Error(String::from(error)),
        }
    }
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
}

fn main() {
    let response_success = ApiResponse::success(
        "Your endpoint works successfully !",
        User {
            name: String::from("Bryan"),
            email: String::from("bryan.cellier.pro@gmail.com"),
        },
    );

    let response_error = ApiResponse::error(
        "An error occured during the fetch.",
        "No user with this id exist.",
    );

    println!("{:#?}", response_success);
    println!("{:#?}", response_error);
}
