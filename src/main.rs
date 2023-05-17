#[macro_use] extern crate rocket;

mod auth;
use auth::BasicAuth;

use rocket::{
    serde::json::{Value, json}, 
    response::status,
};


/// curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic am9objpwYXNzd29yZA=='
#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([
        {"id": 1, "name": "John Doe", "email": "john@doe.com"},
        {"id": 2, "name": "Jane Doe", "email": "jane@doe.com"}
    ])
}

/// curl 127.0.0.1:8000/rustaceans/1 -H 'Authorization: Basic am9objpwYXNzd29yZA=='
#[get("/rustaceans/<id>")]
fn get_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}

/// curl 127.0.0.1:8000/rustaceans -X POST -H "Content-Type: application/json" -d '{"name": "Bob Doe", "email": "bob@doe"}'  -H 'Authorization: Basic am9objpwYXNzd29yZA==' -H 'Authorization: Basic am9objpwYXNzd29yZA=='
#[post("/rustaceans", format="json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "Bob Doe", "email": "bob@doe.com"})
}

/// curl 127.0.0.1:8000/rustaceans/1 -X PUT -H "Content-Type: application/json" -d '{"name": "Put Doe", "email": "ed@doe"}' -H 'Authorization: Basic am9objpwYXNzd29yZA=='
#[put("/rustaceans/<id>", format="json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "Put Doe", "email": "ed@doe.com"})
}

/// curl 127.0.0.1:8000/rustaceans/1 -X DELETE -H 'Authorization: Basic am9objpwYXNzd29yZA=='
#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!({"status": "error", "reason": "Resource was not found."})
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"status": "error", "reason": "Unauthorized"})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            get_rustaceans,
            get_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean,
        ])
        .register("/", catchers![not_found, unauthorized])
}

