#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use models::{NewRustacean, Rustacean};
use repositories::RustaceanRepository as Repo;
use rocket::{
    http::Status,
    response::status::{self, Custom, NoContent},
    serde::json::{json, Json, Value}, fairing::AdHoc, Build, Rocket,
};
use rocket_sync_db_pools::database;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

pub fn handle_err<T, E>(err: diesel::result::Error) -> Custom<Value>
where
    E: std::error::Error,
{
    eprintln!("DB error: {}", err);
    match err {
        diesel::result::Error::NotFound => status::Custom(
            Status::NotFound,
            json!({"status": "error", "reason": "Resource was not found."}),
        ),
        _ => status::Custom(
            Status::InternalServerError,
            json!({"status": "error", "reason": "Internal Server Error"}),
        ),
    }
}

/// curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic Zm9vOmJhcg=='
/// curl 137.184.105.64:8000/rustaceans -H 'Authorization: Basic Zm9vOmJhcg=='
#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        Repo::all(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(handle_err::<Custom<Value>, diesel::result::Error>)
    })
    .await
}

/// curl 127.0.0.1:8000/rustaceans/1 -H 'Authorization: Basic Zm9vOmJhcg=='
/// curl 127.0.0.1:8000/rustaceans/1 -H 'Authorization: Basic am9objpwYXNzd29yZA=='
#[get("/rustaceans/<id>")]
async fn get_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        Repo::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(handle_err::<Custom<Value>, diesel::result::Error>)
    })
    .await
}

/// curl 127.0.0.1:8000/rustaceans -X POST -H "Content-Type: application/json" -d '{"name": "Created Doe", "email": "created@doe.com"}' -H 'Authorization: Basic Zm9vOmJhcg=='
/// curl 127.0.0.1:8000/rustaceans -X POST -H "Content-Type: application/json" -d '{"name": "Jane Doe", "email": "jane@doe.com"}' -H 'Authorization: Basic Zm9vOmJhcg=='
/// curl 127.0.0.1:8000/rustaceans -X POST -H "Content-Type: application/json" -d '{"name": "JOe Doe", "email": "doe@doe.com"}' -H 'Authorization: Basic Zm9vOmJhcg=='
/// curl 127.0.0.1:8000/rustaceans -X POST -H "Content-Type: application/json" -d '{"abajaba": "Bad Person", "nonsense": "bad@person.com"}' -H 'Authorization: Basic Zm9vOmJhcg=='
#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        Repo::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(handle_err::<Custom<Value>, diesel::result::Error>)
    })
    .await
}

/// curl 127.0.0.1:8000/rustaceans/5 -X PUT -H "Content-Type: application/json" -d '{"name": "Updated Doe", "email": "updated@doe.com"}' -H 'Authorization: Basic Zm9vOmJhcg=='
#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        Repo::update(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(handle_err::<Custom<Value>, diesel::result::Error>)
    })
    .await
}

/// Authorized:
/// curl 127.0.0.1:8000/rustaceans/4 -X DELETE -H 'Authorization: Basic Zm9vOmJhcg==' -I
/// Unauthorized:
/// curl 127.0.0.1:8000/rustaceans/1 -X DELETE -H 'Authorization: Basic Zm9vOmJhcg==am9objpwYXNzd29yZA==' -I
/// curl 127.0.0.1:8000/rustaceans/1 -X DELETE -H 'Authorization: Basic abajaba' -I
/// curl 127.0.0.1:8000/rustaceans/1 -X DELETE -I
#[delete("/rustaceans/<id>")]
async fn delete_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        let result = Repo::delete(c, id);

        match result {
            Ok(1) => Ok(NoContent),
            Ok(_) => {
                Err(status::Custom(
                    Status::NotFound,
                    json!({"status": "error", "reason": "Resource was not found."}),
                ))
            }
            Err(e) => {
                Err(status::Custom(
                    Status::InternalServerError,
                    json!({"status": "InternalServerError", "reason": e.to_string()}),
                ))
            }
        }
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!({"status": "error", "reason": "Resource was not found."})
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"status": "error", "rocket::serde - Rust For reason": "Unauthorized"})
}

#[catch(422)]
fn unprocessable() -> Value {
    json!({"status": "error", "reason": "Unprocessable Entity!!!"})
}

#[catch(500)]
fn internal_error() -> Value {
    json!({"status": "error", "reason": "Internal Server Error"})
}

/// TODO Doesn't work because reverting migrations removes schema.rs
async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    let conn = DbConn::get_one(&rocket).await.expect("database connection");
    conn.run(|c| {
        c.run_pending_migrations(MIGRATIONS).expect("migration error");
    }).await;

    rocket
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                get_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean,
            ],
        )
        .register(
            "/",
            catchers![not_found, unauthorized, unprocessable, internal_error],
        )
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("diesel migrations", run_db_migrations))
}
