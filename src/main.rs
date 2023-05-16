#[macro_use] extern crate rocket;

use rocket::{serde::json::{Value, json}, response::status};

/// curl 127.0.0.1/8000:rustaceans
#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([
        {"id": 1, "name": "John Doe", "email": "john@doe.com"},
        {"id": 2, "name": "Jane Doe", "email": "jane@doe.com"}
    ])
}

/// curl 127.0.0.1:8000/rustaceans/1
#[get("/rustaceans/<id>")]
fn get_rustacean(id: i32) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}

/// curl 127.0.0.1:8000/rustaceans -X POST -H "Content-Type: application/json" -d '{"name": "Bob Doe", "email": "bob@doe"}'
#[post("/rustaceans", format="json")]
fn create_rustacean() -> Value {
    json!({"id": 3, "name": "Bob Doe", "email": "bob@doe.com"})
}

/// curl 127.0.0.1:8000/rustaceans/1 -X PUT -H "Content-Type: application/json" -d '{"name": "Put Doe", "email": "ed@doe"}'
#[put("/rustaceans/<id>", format="json")]
fn update_rustacean(id: i32) -> Value {
    json!({"id": id, "name": "Put Doe", "email": "ed@doe.com"})
}

/// curl 127.0.0.1:8000/rustaceans/1 -X DELETE
#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!({"status": "error", "reason": "Resource was not found."})
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
        .register("/", catchers![not_found])
}

