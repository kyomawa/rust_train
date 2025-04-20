use user::User;
use utils::api_response::ApiResponse;

mod user;
mod utils;

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
