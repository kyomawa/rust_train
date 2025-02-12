use actix_web::{ get, post, web, HttpResponse, Responder };
use mongodb::{ Collection, Database };
use crate::models::users::{ User, CreateUserRequest };

#[get("/users")]
pub async fn getUsers(db: web::Data<Database>) -> impl Responder {
    let users = db.collection("users");

    let res = users.find().await;

    match res {
        Ok(_) => HttpResponse::Created().json(new_user),
        Err(_) => HttpResponse::InternalServerError().body("Erreur lors de l'insertion"),
    }
}

#[post("/users")]
pub async fn create_user(
    db: web::Data<Database>,
    user: web::Json<CreateUserRequest>
) -> impl Responder {
    let users: Collection<User> = db.collection("users");

    let new_user = User {
        id: None,
        name: user.name.clone(),
        mobile: user.mobile.clone(),
        email: user.email.clone(),
    };

    let res = users.insert_one(&new_user).await;

    match res {
        Ok(_) => HttpResponse::Created().json(new_user),
        Err(_) => HttpResponse::InternalServerError().body("Erreur lors de l'insertion"),
    }
}
